use std::{sync::Arc, time::Duration};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use sqlx::types::Uuid;
use sqlx::PgPool;
use tower_cookies::{CookieManagerLayer, Cookies, Key};
use tower_http::{
    compression::CompressionLayer, services::ServeDir, timeout::TimeoutLayer, trace::TraceLayer,
};

use super::{
    base::{health, home},
    rooms::{
        create_game_form, create_game_with_auth, create_room_handler, login_handler,
        room_games_page,
    },
};
use crate::domain::room::{GameCreate, RoomCreate};
use crate::routes::rooms::LoginForm;
use crate::services::{GameService, RoomService};
use askama::Template;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
    pub room_service: Arc<RoomService>,
    pub game_service: Arc<GameService>,
    pub cookie_key: Key,
}

pub fn router(
    db: Arc<PgPool>,
    room_service: Arc<RoomService>,
    game_service: Arc<GameService>,
) -> Router {
    let cookie_key = Key::generate();

    let app_state = AppState {
        db,
        room_service: room_service.clone(),
        game_service: game_service.clone(),
        cookie_key,
    };

    // build our application with a route
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .merge(room_routes())
        .merge(authenticated_room_routes())
        .merge(serve_assets())
        .with_state(app_state)
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
            CompressionLayer::new(),
            CookieManagerLayer::new(),
        ))
}

fn serve_assets() -> Router<AppState> {
    // serve the file in the "assets" directory under `assets`
    Router::new().nest_service("/assets", ServeDir::new("assets"))
}

fn room_routes() -> Router<AppState> {
    Router::new()
        .route("/v1/room", post(create_room_handler_route))
        .route("/v1/create_room", get(create_game_form))
        .route("/login", post(login_handler_route))
        .route("/logout", post(logout_handler))
}

fn authenticated_room_routes() -> Router<AppState> {
    Router::new()
        .route("/v1/rooms/{room_id}/games", get(get_room_games_handler))
        .route("/v1/rooms/{room_id}/games", post(create_game_handler_route))
}

// Handler functions
async fn create_room_handler_route(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<RoomCreate>,
) -> impl IntoResponse {
    match create_room_handler(state.room_service, cookies, &state.cookie_key, Form(form)).await {
        Ok(redirect) => redirect.into_response(),
        Err(err) => err.into_response(),
    }
}

async fn login_handler_route(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    login_handler(state.room_service, cookies, &state.cookie_key, Form(form)).await
}

async fn get_room_games_handler(
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>,
    cookies: Cookies,
) -> impl IntoResponse {
    use crate::middleware::validate_room_session;
    use axum::http::StatusCode;

    // Validate signed session cookie first
    if let Err(error_msg) = validate_room_session(&cookies, &state.cookie_key, room_id) {
        let error_template = ErrorPageTemplate {
            title: "Access Denied - RogueVTT".to_string(),
            message: match error_msg {
                "No valid session" => "Please login to access this room".to_string(),
                "Invalid session format" => "Invalid session - please login again".to_string(),
                "Access denied" => "You don't have access to this room".to_string(),
                _ => "Authentication failed - please login".to_string(),
            },
        };
        let status = match error_msg {
            "Access denied" => StatusCode::FORBIDDEN,
            _ => StatusCode::UNAUTHORIZED,
        };
        return (status, crate::routes::Askama(error_template)).into_response();
    }

    // If authenticated, show the room games page
    match room_games_page(state.room_service, state.game_service, Path(room_id)).await {
        Ok(response) => response.into_response(),
        Err(err) => err.into_response(),
    }
}

async fn create_game_handler_route(
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>,
    cookies: Cookies,
    Form(form): Form<GameCreate>,
) -> impl IntoResponse {
    create_game_with_auth(
        state.game_service,
        Path(room_id),
        cookies,
        &state.cookie_key,
        Form(form),
    )
    .await
}

async fn logout_handler(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    use axum::http::{HeaderMap, HeaderValue};
    use tower_cookies::Cookie;

    // Properly remove the room session cookie using make_removal()
    let mut removal_cookie = Cookie::build(("room_session", ""))
        .path("/") // Must match the original cookie path
        .build();
    removal_cookie.make_removal();

    cookies.signed(&state.cookie_key).add(removal_cookie);

    // Use HTMX redirect to send user to homepage
    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", HeaderValue::from_str("/").unwrap());

    (headers, "")
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorPageTemplate {
    title: String,
    message: String,
}
