use crate::domain::event::Event;
use crate::domain::event::Msg;
use crate::domain::game;
use crate::domain::game::GameState;
use crate::event::GameEvent;
use crate::state::memory::Msg::Game;

use async_trait::async_trait;

use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use dashmap::DashMap;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

use crate::domain::game::DTOState;
use tokio::sync::broadcast;
use tokio::sync::mpsc::Sender;

use axum::extract::ws::{Message, WebSocket};

use crate::state::memory::game::Tile;
use futures::{sink::SinkExt, stream::StreamExt};

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
    pub current_level: String,
    pub path: String,
    pub last_save: Option<i64>,
}

pub struct GameChannel {
    pub tx: Arc<broadcast::Sender<String>>,
    pub queue: Vec<Msg>,
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
                let (tx, _rx) = broadcast::channel(100);
                let atx = Arc::new(tx);
                let gc = GameChannel {
                    tx: atx.clone(),
                    queue: Vec::new(),
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

        // TODO ask in the rust channel in axum how to tell when the last websocket closes?
        // state.broadcast_tx.clone().receiver_count()

        // If any one of the tasks run to completion, we abort the other.
        tokio::select! {
            _ = (&mut send_task) => recv_task.abort(), // . here add a print statement, curuous if we can use this + receiver_count to clean out our map later
            _ = (&mut recv_task) => send_task.abort(),
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
            rx: rx,
            loader: loader,
        }
    }

    pub async fn start(&mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                Msg::Game { msg } => {
                    dbg!(msg.data);
                    println!("state store got a message!");
                    ()
                }

                _ => (),
            }
            let ts = get_epoch_ms();
            dbg!(ts);
            println!("Recieved message");
            // dbg!(msg);
            // match message {}
        }
    }
}

#[async_trait]
pub trait Loader {
    async fn get_for_memory(&self, path: String) -> Option<GameState<Vec<Option<u8>>>>;
    async fn get_for_json(&self, path: String) -> Option<GameState<Vec<Tile>>>;
    async fn save_direct(&self, state: &GameState<Vec<Option<u8>>>);
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
        crate::state::db::load_rust(&path).await
    }
    async fn get_for_json(&self, path: String) -> Option<GameState<Vec<Tile>>> {
        crate::state::db::load_json(&path).await
    }
    async fn save_direct(&self, state: &GameState<Vec<Option<u8>>>) {
        crate::state::db::save(state).await
        // probably need some state passed in to make it make sense
        // fn will load the json into memory, apply the queue, then save.
        // ();
    }
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
