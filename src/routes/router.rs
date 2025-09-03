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
    rooms::{create_game_form, create_room_handler},
};
use crate::services::RoomService;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
    pub game_service: Arc<RoomService>,
}

pub fn router(db: Arc<PgPool>, room_service: Arc<RoomService>) -> Router {
    let app_state = AppState {
        db,
        game_service: room_service.clone(),
    };

    // build our application with a route
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .merge(room_routes(room_service.clone()))
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

fn room_routes(room_service: Arc<RoomService>) -> Router<AppState> {
    Router::new()
        .route(
            "/v1/room",
            post(move |form| create_room_handler(room_service, form)),
        )
        .route("/v1/create_room", get(create_game_form))
}
