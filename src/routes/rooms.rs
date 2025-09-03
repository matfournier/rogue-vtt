use std::sync::Arc;

use crate::domain::room::{RoomCreate, RoomResponse};
use crate::routes::{Askama, Result};
use crate::services::RoomService;
use askama::Template;
use axum::{response::IntoResponse, Form, Json};

pub async fn create_game_form() -> impl IntoResponse {
    Askama(CreateRoomTemplate {
        title: "Create Game".to_string(),
    })
}

// Clean handler - no State/Extension knowledge needed
pub async fn create_room_handler(
    room_service: Arc<RoomService>,
    Form(req): Form<RoomCreate>,
) -> Result<Json<RoomResponse>> {
    let response = room_service
        .create_room(req)
        .await
        .map_err(super::error::HttpError::Anyhow)?;
    Ok(Json(response))
}

#[derive(Template)]
#[template(path = "create_room.html")]
struct CreateRoomTemplate {
    title: String,
}
