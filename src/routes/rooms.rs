use std::sync::Arc;

use crate::domain::room::{Auth, GameResponse, RoomCreate, RoomResponse};
use crate::routes::{Askama, Result};
use crate::services::{GameService, RoomService};
use askama::Template;
use axum::{extract::Path, response::{IntoResponse, Redirect}, Form, http::StatusCode};
use sqlx::types::Uuid;

pub async fn create_game_form() -> impl IntoResponse {
    Askama(CreateRoomTemplate {
        title: "Create Game".to_string(),
    })
}

// Clean handler - no State/Extension knowledge needed
pub async fn create_room_handler(
    room_service: Arc<RoomService>,
    Form(req): Form<RoomCreate>,
) -> Result<Redirect> {
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
    
    // Redirect to the room's games page
    Ok(Redirect::to(&format!("/v1/rooms/{}/games", response.id)))
}

pub async fn login_handler(
    room_service: Arc<RoomService>,
    Form(auth): Form<Auth>,
) -> impl IntoResponse {
    // Verify the room credentials
    match room_service.verify_room(&auth).await {
        Ok(()) => {
            // Success - redirect to the room's games page
            Redirect::to(&format!("/v1/rooms/{}/games", auth.id)).into_response()
        }
        Err(_) => {
            // Authentication failed - return error component template
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
