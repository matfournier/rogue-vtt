use crate::domain::game::GameState;
use crate::domain::message::Message;
use crate::domain::{self, game};

use axum::Json;
use dashmap::DashMap;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

pub type VecState = GameState<Vec<Option<u8>>>;

pub struct MemoryReceiver {
    state: Arc<DashMap<String, VecState>>,
    rx: Receiver<Message>,
}
impl MemoryReceiver {
    pub fn make(rx: Receiver<Message>, d: Arc<DashMap<String, VecState>>) -> Self {
        MemoryReceiver { state: d, rx: rx }
    }
}

impl MemoryReceiver {
    pub async fn start(&mut self) {
        while let Some(message) = self.rx.recv().await {
            println!("Recieved a save message!");
            match message {
                Message::EntireGame { game } => {
                    let _ = dbg!(game.clone());
                    let s = self.state.clone();
                    let _ = s.insert(game.id.clone(), game);

                    // self.add(&lvl.id.clone(), lvl);
                }
                Message::TriggerSave { game_id, level_id } => {
                    let s = self.state.clone();
                    let existing_game = s.get(&game_id);
                    dbg!(game_id.clone());
                    if let Some(game) = existing_game {
                        crate::state::db::save(&game).await
                    }
                }
                _ => (),
            }
        }
    }
}

// look into scheduling a save to disk at a regular interval

// let mut interval_timer = tokio::time::interval(chrono::Duration::days(1).to_std().unwrap());
// loop {
//     // Wait for the next interval tick
//     interval_timer.tick().await;
//     tokio::spawn(async { do_my_task().await; }); // For async task
//     tokio::task::spawn_blocking(|| do_my_task()); // For blocking task
// }
pub struct MemoryHandler<T> {
    state: Arc<DashMap<String, T>>,
}

impl<T> MemoryHandler<T>
where
    T: Clone + Serialize,
{
    pub fn make(d: Arc<DashMap<String, T>>) -> Self {
        MemoryHandler { state: d }
    }
    fn add(&self, key: &str, element: T) {
        let w = self.state.clone();
        w.insert(key.to_string(), element);
    }

    fn get_json(&self, key: &str) -> Option<Json<T>> {
        let w = self.state.clone();
        w.get(key).map(|x| Json(x.clone()))
    }
}

impl MemoryHandler<VecState> {
    pub fn create_game(&self, description: &str, xy: &(u32, u32)) -> VecState {
        let gamestate = domain::game::GameState::make(description.to_string(), *xy);
        // TODO: write to durable store here
        // .     also check if it already exists
        let game_id = &gamestate.id.clone();
        self.add(&gamestate.id.clone(), gamestate.clone());
        dbg!(game_id.clone());
        gamestate
    }
    // need to refactor how I store levels for a game, a second map maybe?
    pub async fn get_game_json(&self, game_id: &str, level_id: &str) -> Option<String> {
        println!("here!");
        match self.get_json(game_id) {
            Some(game) => Some(game.toJson()),
            None => {
                let maybe_game = crate::state::db::load(game_id, level_id).await;
                if let Some(game) = maybe_game {
                    self.add(&game.id.clone(), game.clone());
                    Some(game.clone().toJson())
                } else {
                    None
                }
            }
        }
    }
    // TODO: loading a level within an existing game
    // data repr. is all wrong for this.
}
