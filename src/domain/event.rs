use super::game::{DTOState, GameState};

use crate::domain::game::Entity;
use crate::VecState;
use axum::extract::ws::WebSocket;
use serde::{Deserialize, Serialize};

// Todo implement into_iter() for this type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bounds {
    x: u16,
    xx: u16,
    y: u16,
    yy: u16,
}

pub enum Msg {
    Game { msg: GameEvent },
    Internal { event: InternalEvent },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameEvent {
    pub data: Event,
    pub game_id: String,
    pub user: Option<String>,
    pub level: Option<String>,
}
// idea that everything would get sent through the pipe to the handler
// can we send TriggerSave there too? Or should that be inside of the hanlder itself (probably?)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
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
    Fill {
        bounds: Bounds,
        tileset: u16,
        idx: u16,
    },
    Clear {
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

pub enum InternalEvent {
    // Infra Messages
    // InternalSocketJoin {
    //     user: String,
    //     stream: &WebSocket,
    //     game_id: String,
    // },

    // when you hit this:
    //  load map from remote
    //  update map with contents of the buffer
    //  empty the bufer
    //  check if the number of connections in the broadcast channel is 0
    //    delete from the map if true
    //  check last time something was updated
    //    delete if older than 120 minutes ?
    Persist,
}

// https://users.rust-lang.org/t/axum-within-the-standard-chat-example-how-would-you-implement-multiple-chat-rooms/82519/2
// chat room example
// also https://github.com/tokio-rs/axum/blob/main/examples/chat/src/main.rs

pub fn update(event: &GameEvent, state: &mut DTOState) {
    todo!();
}
