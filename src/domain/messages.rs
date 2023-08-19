// https://dev.to/alexeagleson/how-to-set-up-a-fullstack-rust-project-with-axum-react-vite-and-shared-types-429e?

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InitMap {
    kind: i32,
    xy: (i32, i32),
    id: String,
}

impl InitMap {
    pub fn default(id: String) -> InitMap {
        InitMap {
            kind: 9,
            xy: (100, 150),
            id: id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    x: i32,
    y: i32,
    idx: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Level {
    description: String,
    id: String,
    dimension: (i32, i32),
    tiles: Vec<Tile>,
    features: Vec<Tile>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    NPC,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    kind: EntityType,
    x: i32,
    y: i32,
    character: String,
    id: String,
    description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    players: HashMap<String, Entity>,
    npcs: HashMap<String, Entity>,
}
