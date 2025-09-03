use std::{sync::Arc, time::Duration};

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::{
    compression::CompressionLayer, services::ServeDir, timeout::TimeoutLayer, trace::TraceLayer,
};

use super::{
    base::{health, home},
    rooms::{create_game_form, create_room_handler, login_handler, room_games_page},
};
use crate::services::{GameService, RoomService};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
    pub room_service: Arc<RoomService>,
    pub game_service: Arc<GameService>,
}

pub fn router(
    db: Arc<PgPool>,
    room_service: Arc<RoomService>,
    game_service: Arc<GameService>,
) -> Router {
    let app_state = AppState {
        db,
        room_service: room_service.clone(),
        game_service: game_service.clone(),
    };

    // build our application with a route
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .merge(room_routes(&room_service, &game_service))
        .merge(serve_assets())
        .with_state(app_state)
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
            CompressionLayer::new(),
        ))
}

fn serve_assets() -> Router<AppState> {
    // serve the file in the "assets" directory under `assets`
    Router::new().nest_service("/assets", ServeDir::new("assets"))
}

fn room_routes(room_service: &Arc<RoomService>, game_service: &Arc<GameService>) -> Router<AppState> {
    let rs_for_create = room_service.clone();
    let rs_for_login = room_service.clone();
    let rs_for_games = room_service.clone();
    let gs_for_games = game_service.clone();
    
    Router::new()
        .route(
            "/v1/room",
            post(move |form| async move { create_room_handler(rs_for_create, form).await }),
        )
        .route("/v1/create_room", get(create_game_form))
        .route(
            "/login",
            post(move |form| async move { login_handler(rs_for_login, form).await }),
        )
        .route(
            "/v1/rooms/{room_id}/games", 
            get(move |path| async move { room_games_page(rs_for_games, gs_for_games, path).await })
        )
}
