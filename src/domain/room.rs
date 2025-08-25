use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct RoomCreate {
    #[validate(length(min = 1, max = 240))]
    pub game_name: String,
    pub password: String,
}

pub struct RoomAuth {
    pub room_id: Uuid,
    pub password: String,
}

pub struct RoomRow {
    room_id: Uuid,
    game_name: String,
    password_hash: String,
    created_at: DateTime<chrono::Utc>,
}
