// https://dev.to/alexeagleson/how-to-set-up-a-fullstack-rust-project-with-axum-react-vite-and-shared-types-429e?

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InitMap {
    kind: i32,
    xy: (i32, i32),
    id: String,
}

impl InitMap {
    pub fn default(id: String) -> InitMap {
        InitMap {
            kind: 9, // seems wrong?
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
    id: Id,
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
    id: Id,
    description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Id(String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    players: HashMap<Id, Entity>,
    npcs: HashMap<Id, Entity>,
}

impl Entities {
    pub fn to_vec(&self) -> EntityVec {
        EntityVec {
            players: self.players.values().into_iter().cloned().collect(),
            npcs: self.players.values().into_iter().cloned().collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityVec {
    players: Vec<Entity>,
    npcs: Vec<Entity>,
}

impl EntityVec {
    pub fn to_entities(&self) -> Entities {
        let players: HashMap<Id, Entity> = self
            .players
            .clone()
            .into_iter()
            .map(|e| (e.id.clone(), e))
            .collect::<HashMap<_, _>>();
        let npcs: HashMap<Id, Entity> = self
            .npcs
            .clone()
            .into_iter()
            .map(|e| (e.id.clone(), e))
            .collect::<HashMap<_, _>>();

        Entities {
            players: players,
            npcs: npcs,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    level: Level,
    entities: Entities,
}

impl GameState {
    pub fn make(description: String, dimension: (i32, i32)) -> Self {
        let mut players: HashMap<Id, Entity> = HashMap::new();
        players.insert(
            Id("one".to_string()),
            Entity {
                kind: EntityType::Player,
                x: 50,
                y: 50,
                character: "g".to_string(),
                id: Id("shkdfhs".to_string()),
                description: "some_desc".to_string(),
            },
        );
        let npcs: HashMap<Id, Entity> = HashMap::new();
        let entities = Entities {
            players: players,
            npcs: npcs,
        };
        let id = Id(Uuid::new_v4().to_string());
        let level = Level {
            description: description,
            id: id,
            dimension: dimension,
            tiles: vec![Tile {
                x: 25,
                y: 25,
                idx: 20,
            }],
            features: Vec::new(),
        };
        GameState {
            level: level,
            entities: entities,
        }
    }
}
