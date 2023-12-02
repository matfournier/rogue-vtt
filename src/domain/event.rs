use super::game::GameState;

use crate::domain::game::Entity;
use crate::VecState;
use serde::{Deserialize, Serialize};

// Todo implement into_iter() for this type
#[derive(Debug, Serialize, Deserialize)]
pub struct Bounds {
    x_1: u16,
    x_2: u16,
    y_1: u16,
    y_2: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub data: Event,
    pub game_id: String,
    pub user: Option<String>,
    pub level: Option<String>,
}
// idea that everything would get sent through the pipe to the handler
// can we send TriggerSave there too? Or should that be inside of the hanlder itself (probably?)
#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    EntireGame {
        game: VecState,
    },
    TriggerSave {
        game_id: String,
        level_id: String,
    },
    TilePlaced {
        x: u16,
        y: u16,
        tileset: u16,
        idx: u16,
    },
    TileRemoved {
        x: u16,
        y: u16,
        layer: u16,
    },
    FillAction {
        bounds: Bounds,
        tileset: u16,
        idx: u16,
    },
    ClearAction {
        bounds: Bounds,
        layer: u16,
    },
    AddToken {
        entity: Entity,
    },
    RemoveToken {
        entity: Entity,
    },
    MoveToken {
        entity: Entity,
        to: (u32, u32),
    },
    TextMessage {
        user: String,
        msg: String,
    },
}

// https://users.rust-lang.org/t/axum-within-the-standard-chat-example-how-would-you-implement-multiple-chat-rooms/82519/2
// chat room example
// also https://github.com/tokio-rs/axum/blob/main/examples/chat/src/main.rs
