use super::game::GameState;

use crate::domain::game::Entity;
use crate::VecState;

// Todo implement into_iter() for this type
#[derive(Debug)]
pub struct Bounds {
    x_1: u16,
    x_2: u16,
    y_1: u16,
    y_2: u16,
}
// idea that everything would get sent through the pipe to the handler
// can we send TriggerSave there too? Or should that be inside of the hanlder itself (probably?)
#[derive(Debug)]
pub enum Message {
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
}

// https://users.rust-lang.org/t/axum-within-the-standard-chat-example-how-would-you-implement-multiple-chat-rooms/82519/2
// chat room example
// also https://github.com/tokio-rs/axum/blob/main/examples/chat/src/main.rs
