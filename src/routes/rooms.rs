use std::sync::Arc;

use crate::domain::room::{Auth, GameCreate, GameResponse, RoomCreate, RoomResponse};
use serde::Deserialize;
use crate::routes::{Askama, Result};
use crate::middleware::{create_room_session_cookie, validate_room_session, RoomAuth};
use crate::services::{GameService, RoomService};
use askama::Template;
use axum::{extract::Path, response::{IntoResponse, Redirect}, Form, http::{StatusCode, HeaderMap, HeaderValue}, Extension};
use tower_cookies::Cookies;
use std::str::FromStr;
use sqlx::types::Uuid;

#[derive(Deserialize)]
pub struct LoginForm {
    pub id: String,
    pub password: String,
}

pub async fn create_game_form() -> impl IntoResponse {
    Askama(CreateRoomTemplate {
        title: "Create Game".to_string(),
    })
}

// Clean handler - now needs key for signed cookies
pub async fn create_room_handler(
    room_service: Arc<RoomService>,
    cookies: Cookies,
    key: &tower_cookies::Key,
    Form(req): Form<RoomCreate>,
) -> Result<impl IntoResponse> {
    let password = req.password.clone(); // Clone before moving req
    
    let response = room_service
        .create_room(req)
        .await
        .map_err(super::error::HttpError::Anyhow)?;
    
    // Verify the password before redirecting (creator should know the password they just set)
    let auth = Auth {
        id: response.id,
        password,
    };
    
    room_service.verify_room(&auth).await
        .map_err(super::error::HttpError::Anyhow)?;
    
    // Set signed room session cookie
    let session_cookie = create_room_session_cookie(response.id);
    cookies.signed(key).add(session_cookie);
    
    // Use HTMX client-side redirect to allow cookie to be processed
    let redirect_url = format!("/v1/rooms/{}/games", response.id);
    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", HeaderValue::from_str(&redirect_url).unwrap());
    
    Ok((headers, "").into_response())
}

pub async fn login_handler(
    room_service: Arc<RoomService>,
    cookies: Cookies,
    key: &tower_cookies::Key,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    // First validate UUID format
    let room_id = match Uuid::from_str(&form.id) {
        Ok(uuid) => uuid,
        Err(_) => {
            // Invalid UUID format
            let error_template = ErrorAlertTemplate {
                message: "Room should be a UUID".to_string(),
            };
            return (StatusCode::BAD_REQUEST, Askama(error_template)).into_response();
        }
    };
    
    // Create Auth struct with validated UUID
    let auth = Auth {
        id: room_id,
        password: form.password,
    };
    
    // Verify the room credentials
    match room_service.verify_room(&auth).await {
        Ok(()) => {
            // Success - set signed room session cookie and redirect
            let session_cookie = create_room_session_cookie(auth.id);
            cookies.signed(key).add(session_cookie);
            
            Redirect::to(&format!("/v1/rooms/{}/games", auth.id)).into_response()
        }
        Err(_) => {
            // Authentication failed - room doesn't exist or wrong password
            let error_template = ErrorAlertTemplate {
                message: "Invalid Room ID or Password".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Askama(error_template)).into_response()
        }
    }
}

pub async fn room_games_page(
    room_service: Arc<RoomService>,
    game_service: Arc<GameService>,
    Path(room_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    // Get room details
    let room = room_service.get_room(room_id).await
        .map_err(super::error::HttpError::Anyhow)?;
    
    // Get games for this room
    let games = game_service.list_games_in_room(room_id).await
        .map_err(super::error::HttpError::Anyhow)?;

    Ok(Askama(RoomGamesTemplate {
        title: "RogueVTT".to_string(),
        room,
        games,
    }))
}

pub async fn create_game_handler(
    game_service: Arc<GameService>,
    Extension(room_auth): Extension<RoomAuth>,
    Form(mut game_req): Form<GameCreate>,
) -> impl IntoResponse {
    // Ensure the game belongs to the authenticated room
    game_req.room_id = room_auth.room_id;
    
    match game_service.create_game(game_req).await {
        Ok(game) => {
            // Return the game card HTML for HTMX to append
            let game_template = GameCardTemplate { game };
            Askama(game_template).into_response()
        }
        Err(_) => {
            let error_template = ErrorAlertTemplate {
                message: "Failed to create game".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Askama(error_template)).into_response()
        }
    }
}

#[derive(Template)]
#[template(path = "create_room.html")]
struct CreateRoomTemplate {
    title: String,
}

#[derive(Template)]
#[template(path = "room_games.html")]
struct RoomGamesTemplate {
    title: String,
    room: RoomResponse,
    games: Vec<GameResponse>,
}

#[derive(Template)]
#[template(path = "components/error_alert.html")]
struct ErrorAlertTemplate {
    message: String,
}

#[derive(Template)]
#[template(path = "components/game_card.html")]
struct GameCardTemplate {
    game: GameResponse,
}

pub async fn create_game_with_auth(
    game_service: Arc<GameService>,
    Path(room_id): Path<Uuid>,
    cookies: Cookies,
    key: &tower_cookies::Key,
    Form(mut game_req): Form<GameCreate>,
) -> impl IntoResponse {
    // Validate signed session cookie
    if let Err(error_msg) = validate_room_session(&cookies, key, room_id) {
        let error_template = ErrorAlertTemplate {
            message: match error_msg {
                "No valid session" => "Authentication required".to_string(),
                "Invalid session format" => "Invalid session".to_string(), 
                "Access denied" => "Access denied".to_string(),
                _ => "Authentication failed".to_string(),
            },
        };
        let status = match error_msg {
            "Access denied" => StatusCode::FORBIDDEN,
            _ => StatusCode::UNAUTHORIZED,
        };
        return (status, Askama(error_template)).into_response();
    }

    // Set the room_id in the game request
    game_req.room_id = room_id;

    // Create the game
    match game_service.create_game(game_req).await {
        Ok(game) => {
            let game_template = GameCardTemplate { game };
            Askama(game_template).into_response()
        }
        Err(_) => {
            let error_template = ErrorAlertTemplate {
                message: "Failed to create game".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Askama(error_template)).into_response()
        }
    }
}
