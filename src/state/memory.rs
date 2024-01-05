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

pub type VecState = GameState<Vec<Option<u16>>>;

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

pub struct SocketConnector {
    pub state: Arc<DashMap<String, GameChannel>>,
    pub loader: Arc<dyn Loader + Send + Sync>,
    pub sender: Arc<Sender<Msg>>,
}

impl SocketConnector {
    pub async fn connect(&self, game_id: &str, socket: WebSocket, _user: &str) {
        let s = self.state.clone();
        tracing::info!(
            "connecting new user: {:?} to game_id {:?}",
            &game_id,
            &_user
        );
        let game = s.get(game_id);
        let broadcast_tx = match game {
            Some(g) => g.tx.clone(),
            None => {
                let (tx, _rx) = broadcast::channel(150);
                let atx = Arc::new(tx);
                let gc = GameChannel {
                    tx: atx.clone(),
                    metadata: GameMetadata {
                        current_level: "todo".to_string(),
                        path: "todo".to_string(),
                        last_save: None,
                    },
                };
                s.insert(game_id.to_string(), gc);
                atx.clone()
            }
        };

        let (mut sender, mut receiver) = socket.split();

        let mut rx = broadcast_tx.subscribe();

        // Now send the "joined" message to all subscribers.
        let zzz = Event::TextMessage {
            user: "server".to_string(),
            msg: format!("UNKNOWN joined."),
        };

        let msg = serde_json::to_string(&zzz).unwrap();
        tracing::debug!("{msg}");
        let _ = broadcast_tx.send(msg);

        // Spawn the first task that will receive broadcast messages and send text
        // messages over the websocket to our client.
        let mut send_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                // In any websocket error, break loop.
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });

        // Clone things we want to pass (move) to the receiving task.
        let tx = broadcast_tx.clone();
        // let name = username.clone();

        // Spawn a task that takes messages from the websocket,
        // deserializes the event and dispatches it to every other connected websocket
        // janky error handling TODO: add in proper logging

        let gtid: String = game_id.to_string();
        let sender = self.sender.clone();
        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                // dbg!(text.clone());
                match serde_json::from_str::<Event>(&text) {
                    Ok(v) => {
                        // I wonder if this should go in it's own thread?
                        // weird when I tried to spawn a thread to do this get tons
                        // of ownership issues on gtid and sender
                        let msg = Game {
                            msg: GameEvent {
                                data: v,
                                game_id: gtid.clone(),
                                user: None,
                                level: None,
                            },
                        };
                        let socket_msg = match sender.try_send(msg) {
                            Ok(_) => text,
                            Err(_) => "ERROR: could not send message to server for state storage"
                                .to_string(),
                        };

                        let _ = tx.send(socket_msg);
                    }
                    Err(e) => {
                        dbg!(text);
                        tracing::error!(
                            "Something went wrong with {:?}, error: {:?}",
                            gtid.clone(),
                            &e
                        )
                    }
                }
            }
        });

        // If any one of the tasks run to completion, we abort the other.
        tokio::select! {
            _ = (&mut send_task) => {
                // no way to block on this which is why we see a log message of 1
                recv_task.abort();
                tracing::info!("recv_task abort for game {:?} user {:?} count: {:?}", game_id.to_string(), &_user, broadcast_tx.receiver_count());
            }, // . here add a print statement, curuous if we can use this + receiver_count to clean out our map later
            _ = (&mut recv_task) => {
                send_task.abort();
                tracing::info!("send_task abort for game {:?} user {:?} count: {:?}", game_id.to_string(), &_user, broadcast_tx.receiver_count());
            }
        };
    }
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
