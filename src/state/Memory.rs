use crate::domain::event::Event;
use crate::domain::event::Msg;
use crate::domain::game::GameState;
use crate::event::GameEvent;
use crate::event::InternalEvent;
use crate::state::memory::Msg::Game;
use crate::Loader;
use std::collections::HashMap;

use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use dashmap::DashMap;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

use std::thread;

use crate::domain::game::DTOState;
use tokio::sync::broadcast;
use tokio::sync::mpsc::Sender;

use axum::extract::ws::{Message, WebSocket};

use futures::future::TryFutureExt;
use futures::stream;
use futures::{sink::SinkExt, stream::StreamExt};

pub type VecState = GameState<Vec<Option<u16>>>;

// idea: we keep the last ? 1 ? 2 ? 3 minutes of events
// we have a tokio.interval that sends an event to sink everything that needs to be sunk (e.g. queue is not empty)
// to storage (it does not call the method directly).  It also deletes the queue + checks if tx is empty and deletes the key if so
// problem: how to handle when someone connects + we are deleting queue at the same time?
//  someone connects -> gets object + queue but I think we have a race condition?
//  also what about events when they are joining that they might miss? another race condition
//    - we could dispatch all events to clients with timestamps, and they could filter the ones they don't need when someone joins?
//    - since every event would have a state with a timestamp maybe?   IMHO the timestamp should come from the server?

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
    pub async fn connect(&self, game_id: &str, socket: WebSocket, user: &str) {
        let s = self.state.clone();
        // todo make sure the game already exists otherwise fail.
        println!("CONNECTING");
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
                dbg!(text.clone());
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
                    Err(_) => {
                        println!("something went wrong with");
                        dbg!(text);
                        println!("above---^");
                    }
                }
            }
        });

        // If any one of the tasks run to completion, we abort the other.
        tokio::select! {
            _ = (&mut send_task) => {
                recv_task.abort();
                println!("send_task abort!");
                println!("count: {}", broadcast_tx.receiver_count());
            }, // . here add a print statement, curuous if we can use this + receiver_count to clean out our map later
            _ = (&mut recv_task) => {
                send_task.abort();
                println!("recv_task abort!");
                println!("count: {}", broadcast_tx.receiver_count());
            }
        };

        // Send "user left" message (similar to "joined" above).
        // let msg = format!("{username} left.");
        // tracing::debug!("{msg}");
        // let _ = state.tx.send(msg);

        // Remove username from map so new clients can take it again.
        // state.user_set.lock().unwrap().remove(&username);
        println!("websocket connected!")
    }
}

// if we wanted to increase parallism we could do make N of these
// and do some hashcode -> N
// Map<Int, Dispatcher> that is deterministic on GameId
// that way we would have many channels at once
// for now assume single threaded
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
                    println!("state store got a message!");
                    ()
                }
                Msg::Internal { event } => {
                    println!("state store got an internal message! started saving!");

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

                            println!("finished saving!");

                            // TODO here should check the number of subscribers
                            // if it is zero delete it from both maps

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

                            println!("do I get here?");
                            let _ = keys.into_iter().for_each(|k| {
                                println!("{}", format!("processing {}", k.to_string()));
                                let gc = self.state.get_mut(&k).unwrap();
                                let tx = gc.tx.clone();
                                let subscribers = tx.receiver_count();
                                let last_save: u128 = match gc.metadata.last_save.clone() {
                                    Some(v) => v,
                                    None => 0,
                                };
                                dbg!(last_save);
                                // why is this 1 and not zero?
                                println!("{}", format!("subscriber count {}", subscribers));
                                if subscribers <= 0 {
                                    println!("DISCONNECTING!!!!! test");
                                    self.state.remove(&k);
                                };
                                // change this to some larger value
                                // if get_epoch_ms() - last_save > 60000 {
                                //     println!("CLOSING DUE TO INACTIVITY!");
                                //     // thid eson't work since broadcast channels don't have a close method
                                //     // let mut zz = gc.rx;
                                //     // zz.close();
                                // };
                            });
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
