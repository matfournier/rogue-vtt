use crate::domain::event::Event;
use crate::domain::event::Msg;
use crate::domain::game::GameState;
use crate::event::GameEvent;
use crate::event::InternalEvent;
use crate::state::memory::Msg::Game;
use crate::Loader;
use std::collections::HashMap;

use std::time::{SystemTime, UNIX_EPOCH};

use dashmap::DashMap;

use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

use tokio::sync::broadcast;
use tokio::sync::mpsc::Sender;

use axum::extract::ws::{Message, WebSocket};

use futures::{sink::SinkExt, stream::StreamExt};

pub type VecState = GameState<Vec<Option<i16>>>;

pub struct GameMetadata {
    pub current_level: String,
    pub path: String,
    pub last_save: Option<u128>,
}

impl GameMetadata {
    pub fn save(&mut self) {
        self.last_save = Some(get_epoch_ms());
    }
}

pub struct GameChannel {
    pub tx: Arc<broadcast::Sender<String>>,
    // how do we handle level changes?
    // changing level writes to disk and updates everything maybe?
    pub metadata: GameMetadata,
}

pub struct Dispatcher {
    // GameId -> GameChannel
    state: Arc<DashMap<String, GameChannel>>,
    game_events: HashMap<String, Vec<GameEvent>>,
    rx: Receiver<Msg>,
    loader: Arc<dyn Loader + Send + Sync>,
}

impl Dispatcher {
    pub fn make(
        rx: Receiver<Msg>,
        loader: Arc<dyn Loader + Send + Sync>,
        state: Arc<DashMap<String, GameChannel>>,
    ) -> Self {
        Dispatcher {
            state: state,
            game_events: HashMap::new(),
            rx: rx,
            loader: loader,
        }
    }

    // read https://stackoverflow.com/questions/68233404/wrapper-struct-for-hashmap-mutex-rust
    // and https://stackoverflow.com/a/65434321

    pub async fn start(&mut self) {
        while let Some(msg_container) = self.rx.recv().await {
            match msg_container {
                Msg::Game { msg } => {
                    // if this deadlocks consider making two different DashMaps
                    // one for GameChannel and one for the queue
                    // if let Some(mut state) = self.state.get_mut(&msg.game_id) {
                    //     state.put(msg)
                    // }

                    if let Some(mut _state) = self.state.get_mut(&msg.game_id) {
                        match self.game_events.get_mut(&msg.game_id) {
                            Some(vec) => vec.push(msg),
                            None => {
                                self.game_events.insert(msg.game_id.clone(), vec![msg]);
                                ()
                            }
                        }
                    }
                    // dbg!(msg.data);
                    // println!("state store got a message!");
                    ()
                }
                Msg::Internal { event } => {
                    tracing::info!("State store saving triggered");

                    match event {
                        InternalEvent::Persist => {
                            let res = futures::stream::iter(self.game_events.drain().map(
                                |(game_id, queue)| async move {
                                    if queue.len() != 0 {
                                        // load the file from disk
                                        let path = crate::loader::path_with_game(&game_id);

                                        // todo error handling
                                        let mut game =
                                            crate::state::db::load_rust(&path).await.unwrap();

                                        // then iterate over the queue and update the game
                                        queue.into_iter().for_each(|msg| game.update_with(&msg));
                                        crate::state::db::save(&game).await;

                                        // cannot update the save time annoyingly so not sure how I'm going to
                                        // get rid of OLD connections where no game action has happened in some time

                                        // self.state.get_mut(&game_id).unwrap().metadata.save();
                                        Some(game_id.to_string())
                                    } else {
                                        None
                                    }
                                },
                            ))
                            .buffer_unordered(4);

                            let games_to_update_time: Vec<Option<String>> =
                                res.collect::<Vec<_>>().await;

                            games_to_update_time.iter().for_each(|game| match game {
                                Some(g) => {
                                    let mut gc = self.state.get_mut(g).unwrap();
                                    gc.metadata.save();
                                }
                                None => (),
                            });

                            // for replaying state... make a multistat event and send it to clients
                            // clients need to store their initial state in order for people to replay
                            // will need a new event for them to update their initial state too
                            // e.g. after things get saved.
                            let keys: Vec<String> = self
                                .state
                                .clone()
                                .iter()
                                .map(|r| r.key().to_string())
                                .collect();

                            let _ = keys.into_iter().for_each(|k| {
                                println!("{}", format!("processing {}", k.to_string()));
                                let gc = self.state.get_mut(&k).unwrap();
                                let tx = gc.tx.clone();
                                let subscribers = tx.receiver_count();
                                // let last_save: u128 = match gc.metadata.last_save.clone() {
                                //     Some(v) => v,
                                //     None => 0,
                                // };
                                if subscribers <= 0 {
                                    tracing::info!(
                                        "Cleaning up {:?} due to zero active subscribers",
                                        &k
                                    );
                                    self.state.remove(&k);
                                    self.game_events.remove(&k);
                                };
                            });
                            tracing::info!("State store saving finished");
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
