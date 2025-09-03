use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct RoomCreate {
    #[validate(length(min = 1, max = 240))]
    pub name: String,
    pub password: String,
}

pub struct Auth {
    pub id: Uuid,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
    pub created_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct RoomResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<chrono::Utc>,
}

impl From<Room> for RoomResponse {
    fn from(room: Room) -> Self {
        Self {
            id: room.id,
            name: room.name,
            created_at: room.created_at,
        }
    }
}

#[derive(Deserialize, Serialize, Validate)]
pub struct GameCreate {
    #[validate(length(min = 1, max = 240))]
    pub name: String,
}
