// https://dev.to/alexeagleson/how-to-set-up-a-fullstack-rust-project-with-axum-react-vite-and-shared-types-429e?

use crate::event::Event;
use serde::Deserialize;
use serde::Serialize;
use serde_repr::*;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use uuid::Uuid;

use super::event::GameEvent;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    x: u16,
    y: u16,
    idx: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LevelType {
    Dungeon = 0,
    Overland = 1,
}

pub enum TileType {
    Dungeon,
    Feature,
    // All, // not sure about this one!
}

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub struct Level {
//     pub kind: LevelType,
//     pub description: String,
//     pub id: Id,
//     pub dimension: (i32, i32),
//     pub tiles: HashMap<(i32, i32), Tile>, // this is really poor for removing tiles imho, need to change this into a Map
//     pub features: HashMap<(i32, i32), Tile>, // this is really poor for removing tiles imho, need to change this into a Map
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Level<T> {
    pub kind: LevelType,
    pub description: String,
    pub id: Id,
    pub dimension: (u16, u16),
    pub tiles: T,
    pub features: T,
}

impl<T> Level<T> {
    pub fn pointToIdx(&self, x: &u16, y: &u16) -> Option<u32> {
        if *x > self.dimension.0 || *y > self.dimension.1 {
            None
        } else {
            u32::try_from(*y * self.dimension.1 + *x).ok()
        }
    }

    pub fn idxToPoint(&self, idx: u32) -> (u16, u16) {
        let x = idx % self.dimension.1 as u32;
        let y = idx / self.dimension.1 as u32;
        (x as u16, y as u16)

        // idxToCoords(idx: number): [number, number] {
        //     let x = idx % this.width
        //     let y = Math.floor(idx / this.width)
        //     return [x, y]
        // }
    }
}

// if let Some(pos) = self.level.pointToIdx(&t.x, &t.y) {
//     dungeon[pos as usize] = Some(t.idx.clone());
// }
impl Level<Vec<Option<u16>>> {
    pub fn add(&mut self, x: u16, y: u16, tileset: u16, idx: u16) {
        if let Some(pos) = self.pointToIdx(&x, &y) {
            if tileset == 0 {
                self.tiles[pos as usize] = Some(idx)
            } else {
                self.features[pos as usize] = Some(idx)
            }
        }
    }

    pub fn remove(&mut self, x: u16, y: u16, tileset: u16) {
        if let Some(idx) = self.pointToIdx(&x, &y) {
            if tileset == 0 {
                self.tiles[idx as usize] = None
            } else if tileset == 1 {
                self.features[idx as usize] = None
            } else {
                self.tiles[idx as usize] = None;
                self.features[idx as usize] = None
            }
        }
    }
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
pub struct GameState<T> {
    pub level: Level<T>,
    pub entities: Entities,
    pub id: String,
}

pub type DTOState = GameState<Vec<Tile>>;

impl GameState<Vec<Tile>> {
    pub fn toRust(&self) -> GameState<Vec<Option<u16>>> {
        let size: u32 = self.level.dimension.0 as u32 * self.level.dimension.1 as u32;
        let mut dungeon: Vec<Option<u16>> = Vec::with_capacity(size.clone() as usize);
        let mut features: Vec<Option<u16>> = Vec::with_capacity(size.clone() as usize);
        for i in 0..size {
            dungeon.push(None);
            features.push(None);
        }
        // now iterate through each level and feature filling in what you need to on this array
        // and remake the GameState object
        self.level.tiles.clone().into_iter().for_each(|t| {
            if let Some(pos) = self.level.pointToIdx(&t.x, &t.y) {
                dungeon[pos as usize] = Some(t.idx.clone());
            }
        });

        self.level.features.clone().into_iter().for_each(|t| {
            if let Some(pos) = self.level.pointToIdx(&t.x, &t.y) {
                features[pos as usize] = Some(t.idx.clone());
            }
        });

        let new_level = Level {
            id: self.level.id.clone(),
            kind: self.level.kind.clone(),
            description: self.level.description.clone(),
            dimension: self.level.dimension.clone(),
            tiles: dungeon,
            features: features,
        };

        GameState {
            level: new_level,
            entities: self.entities.clone(),
            id: self.id.clone(),
        }
    }
}

// I wish I had Map<string, tile> but interop w/ typescript is a pain
// this seems VERY wasteful but who knows
impl GameState<Vec<Option<u16>>> {
    pub fn make(description: String, dimension: (u16, u16)) -> Self {
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
            tiles: Vec::new(),
            features: Vec::new(),
        };
        GameState {
            level: level,
            entities: entities,
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn toJson(&self) -> String {
        // here convert to GameState<Vec<Tile>> perhaps ?
        // and then conver that to JSON?
        let mut level_dungeon: Vec<Tile> = Vec::new();
        let mut level_features: Vec<Tile> = Vec::new();

        self.level
            .tiles
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, tile)| match tile {
                Some(tile_idx) => {
                    let pt = self.level.idxToPoint(i as u32);
                    level_dungeon.push(Tile {
                        x: pt.0,
                        y: pt.1,
                        idx: tile_idx,
                    })
                }
                None => (),
            });

        self.level
            .features
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, tile)| match tile {
                Some(tile_idx) => {
                    let pt = self.level.idxToPoint(i as u32);
                    level_features.push(Tile {
                        x: pt.0,
                        y: pt.1,
                        idx: tile_idx,
                    })
                }
                None => (),
            });

        let new_level: Level<Vec<Tile>> = Level {
            id: self.level.id.clone(),
            kind: self.level.kind.clone(),
            description: self.level.description.clone(),
            dimension: self.level.dimension.clone(),
            tiles: level_dungeon,
            features: level_features,
        };

        let new_gs = GameState {
            level: new_level,
            entities: self.entities.clone(),
            id: self.id.clone(),
        };

        serde_json::to_string(&new_gs).unwrap()
    }

    // TODO clean up that idx is u16 and not u8
    pub fn addTile(&mut self, x: u16, y: u16, tileset: u16, idx: u16) {
        self.level.add(x, y, tileset, idx)
    }

    pub fn removeTile(&mut self, x: u16, y: u16, tileset: u16) {
        self.level.remove(x, y, tileset)
    }

    pub fn update_with(&mut self, event: &GameEvent) {
        match &event.data {
            Event::TilePlaced { x, y, tileset, idx } => self.addTile(*x, *y, *tileset, *idx),
            Event::TileRemoved { x, y, layer } => self.removeTile(*x, *y, *layer),
            Event::Fill {
                bounds,
                tileset,
                idx,
            } => bounds
                .vec()
                .into_iter()
                .for_each(|(x, y)| self.addTile(x, y, *tileset, *idx)),
            _ => (),
        };
    }
}
