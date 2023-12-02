use crate::domain::event::Message;
use crate::domain::game;
use crate::domain::game::GameState;

use async_trait::async_trait;

use axum::Json;
use dashmap::DashMap;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

use crate::domain::game::DTOState;
use tokio::sync::broadcast;

use crate::state::memory::game::Tile;

pub type VecState = GameState<Vec<Option<u8>>>;

// idea: we keep the last ? 1 ? 2 ? 3 minutes of events
// we have a tokio.interval that sends an event to sink everything that needs to be sunk (e.g. queue is not empty)
// to storage (it does not call the method directly).  It also deletes the queue + checks if tx is empty and deletes the key if so
// problem: how to handle when someone connects + we are deleting queue at the same time?
//  someone connects -> gets object + queue but I think we have a race condition?
//  also what about events when they are joining that they might miss? another race condition
//    - we could dispatch all events to clients with timestamps, and they could filter the ones they don't need when someone joins?
//    - since every event would have a state with a timestamp maybe?   IMHO the timestamp should come from the server?

pub struct GameMetadata {
    pub currentLevel: String,
    pub path: String,
    pub lastSave: Option<i64>,
}

pub struct GameChannel {
    pub tx: broadcast::Sender<String>,
    pub queue: Vec<Message>,
    // how do we handle level changes?
    // changing level writes to disk and updates everything maybe?
    pub metadata: GameMetadata,
}

// if we wanted to increase parallism we could do make N of these
// and do some hashcode -> N
// Map<Int, Dispatcher> that is deterministic on GameId
// that way we would have many channels at once
// for now assume single threaded
pub struct Dispatcher {
    // GameId -> GameChannel
    state: Arc<DashMap<String, GameChannel>>,
    rx: Receiver<Message>,
    loader: Arc<dyn Loader + Send + Sync>,
}

#[async_trait]
pub trait Loader {
    async fn get_for_memory(&self, path: String) -> Option<GameState<Vec<Option<u8>>>>;
    async fn get_for_json(&self, path: String) -> Option<GameState<Vec<Tile>>>;
    async fn save(&self, path: String);
    fn path_with_game(&self, game_id: &str) -> String {
        crate::state::db::game_to_path(game_id)
    }
    fn path_with_level(&self, game_id: &str, level_id: &str) -> String {
        crate::state::db::game_level_to_path(game_id, level_id)
    }
}

pub struct LocalLoader;

#[async_trait]
impl Loader for LocalLoader {
    async fn get_for_memory(&self, path: String) -> Option<GameState<Vec<Option<u8>>>> {
        None
    }
    async fn get_for_json(&self, path: String) -> Option<GameState<Vec<Tile>>> {
        None
    }
    async fn save(&self, path: String) {
        ();
    }
}

impl Dispatcher {
    pub fn make(rx: Receiver<Message>, loader: Arc<dyn Loader + Send + Sync>) -> Self {
        Dispatcher {
            state: Arc::new(DashMap::new()),
            rx: rx,
            loader: loader,
        }
    }

    pub async fn start(&mut self) {
        while let Some(msg) = self.rx.recv().await {
            println!("Recieved message");
            dbg!(msg);
            // match message {}
        }
    }
}

// pub struct MemoryReceiver {
//     state: Arc<DashMap<String, VecState>>,
//     rx: Receiver<Event>,
// }
// impl MemoryReceiver {
//     pub fn make(rx: Receiver<Event>, d: Arc<DashMap<String, VecState>>) -> Self {
//         MemoryReceiver { state: d, rx: rx }
//     }
// }

// impl MemoryReceiver {
//     pub async fn start(&mut self) {
//         while let Some(message) = self.rx.recv().await {
//             println!("Recieved a save message!");
//             match message {
//                 Event::EntireGame { game } => {
//                     let _ = dbg!(game.clone());
//                     let s = self.state.clone();
//                     let _ = s.insert(game.id.clone(), game);

//                     // self.add(&lvl.id.clone(), lvl);
//                 }
//                 Event::TriggerSave { game_id, level_id } => {
//                     let s = self.state.clone();
//                     let existing_game = s.get(&game_id);
//                     dbg!(game_id.clone());
//                     if let Some(game) = existing_game {
//                         crate::state::db::save(&game).await
//                     }
//                 }
//                 _ => (),
//             }
//         }
//     }
// }

// // look into scheduling a save to disk at a regular interval

// // let mut interval_timer = tokio::time::interval(chrono::Duration::days(1).to_std().unwrap());
// // loop {
// //     // Wait for the next interval tick
// //     interval_timer.tick().await;
// //     tokio::spawn(async { do_my_task().await; }); // For async task
// //     tokio::task::spawn_blocking(|| do_my_task()); // For blocking task
// // }
// pub struct MemoryHandler<T> {
//     state: Arc<DashMap<String, T>>,
// }

// impl<T> MemoryHandler<T>
// where
//     T: Clone + Serialize,
// {
//     pub fn make(d: Arc<DashMap<String, T>>) -> Self {
//         MemoryHandler { state: d }
//     }
//     fn add(&self, key: &str, element: T) {
//         let w = self.state.clone();
//         w.insert(key.to_string(), element);
//     }

//     fn get_json(&self, key: &str) -> Option<Json<T>> {
//         let w = self.state.clone();
//         w.get(key).map(|x| Json(x.clone()))
//     }
// }

// impl MemoryHandler<VecState> {
//     pub fn create_game(&self, description: &str, xy: &(u32, u32)) -> VecState {
//         let gamestate = domain::game::GameState::make(description.to_string(), *xy);
//         // TODO: write to durable store here
//         // .     also check if it already exists
//         let game_id = &gamestate.id.clone();
//         self.add(&gamestate.id.clone(), gamestate.clone());
//         dbg!(game_id.clone());
//         gamestate
//     }
//     // need to refactor how I store levels for a game, a second map maybe?
//     pub async fn get_game_json(&self, game_id: &str, level_id: &str) -> Option<String> {
//         println!("here!");
//         match self.get_json(game_id) {
//             Some(game) => Some(game.toJson()),
//             None => {
//                 let maybe_game = crate::state::db::load(game_id, level_id).await;
//                 if let Some(game) = maybe_game {
//                     self.add(&game.id.clone(), game.clone());
//                     Some(game.clone().toJson())
//                 } else {
//                     None
//                 }
//             }
//         }
//     }
//     // TODO: loading a level within an existing game
//     // data repr. is all wrong for this.
// }
