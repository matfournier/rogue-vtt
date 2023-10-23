// https://dev.to/alexeagleson/how-to-set-up-a-fullstack-rust-project-with-axum-react-vite-and-shared-types-429e?

use serde::Deserialize;
use serde::Serialize;
use serde_repr::*;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use uuid::Uuid;

// #[derive(Clone, Debug, Default, PartialEq, Serialize)]
// pub struct InitMap {
//     kind: i32,
//     xy: (i32, i32),
//     description: String,
// }

// impl InitMap {
//     pub fn default(id: String, description: String) -> InitMap {
//         InitMap {
//             kind: 9, // seems wrong?
//             xy: (100, 150),
//             description: String,
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    x: i32,
    y: i32,
    idx: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LevelType {
    Dungeon = 0,
    Overland = 1,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub kind: LevelType,
    pub description: String,
    pub id: Id,
    pub dimension: (i32, i32),
    pub tiles: Vec<Tile>,
    pub features: Vec<Tile>,
}

#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EntityType {
    Player = 0, // this is turning weird on the json, I'm geting string "player" rather than 0
    NPC = 1,
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
pub struct Id(String);

impl Id {
    pub fn new(val: String) -> Self {
        Id(val)
    }
}

impl FromStr for Id {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Id(s.to_string()))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Id {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

// Each level has an Id
//  - use this to switch levels within a game
// Each Game has an Id
//  - use this to manage game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub level: Level,
    pub entities: Entities,
    pub id: String,
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
        let level = Level {
            kind: LevelType::Dungeon,
            description: description,
            id: Id(Uuid::new_v4().to_string()),
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
            id: Uuid::new_v4().to_string(),
        }
    }
}
