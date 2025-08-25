use std::time::Duration;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;
use tower_http::{services::ServeDir, timeout::TimeoutLayer, trace::TraceLayer};

use super::{
    base::{health, home},
    rooms::create_room,
};

pub fn router(db: PgPool) -> Router {
    // build our application with a route
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .merge(room_routes())
        .merge(serve_assets())
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .layer(Extension(db))
}

fn serve_assets() -> Router {
    // serve the file in the "assets" directory under `assets`
    Router::new().nest_service("/assets", ServeDir::new("assets"))
}

fn room_routes() -> Router {
    Router::new().route("/v1/room", post(create_room))
}
