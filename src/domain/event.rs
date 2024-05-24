use crate::domain::game::Entity;
use crate::VecState;
use serde::{Deserialize, Serialize};

// Todo implement into_iter() for this type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bounds {
    x: i16,
    xx: i16,
    y: i16,
    yy: i16,
}

impl Bounds {
    pub fn vec(&self) -> Vec<(i16, i16)> {
        let mut v: Vec<(i16, i16)> = Vec::new();
        for x in self.x..=self.xx {
            for y in self.y..=self.yy {
                v.push((x, y));
            }
        }
        v
    }
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
        x: i16,
        y: i16,
        tileset: i16,
        idx: i16,
    },
    TileRemoved {
        x: i16,
        y: i16,
        layer: i16,
    },
    Fill {
        bounds: Bounds,
        tileset: i16,
        idx: i16,
    },
    Clear {
        bounds: Bounds,
        layer: i16,
    },
    AddToken {
        entity: Entity,
    },
    RemoveToken {
        entity: Entity,
    },
    MoveToken {
        entity: Entity,
        to: (i32, i32),
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
    // MulityEvent { events: Vec<GameEvent> },
}
