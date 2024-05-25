use crate::domain::event::Event;
use serde_json;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::AtomicI8;
use tokio::sync::RwLock;
use tokio_stream::{self as stream};

use dashmap::DashMap;
use std::sync::Arc;

use crate::domain::event::GameEvent;
use crate::event::Msg;
use axum::extract::ws::{Message, WebSocket};
use std::sync::atomic::Ordering::Relaxed;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use futures::{sink::SinkExt, stream::StreamExt};

pub struct Room {
    pub messages: RwLock<Vec<GameEvent>>,
    pub current_level: String,
    pub last_save: Option<u32>,
    pub path: String,
    // rx: Receiver<Msg>,
    // I feel like it should have the broadcast channel for the websocket?
    //     // Channel used to send messages to all connected clients.
    // see https://github.com/tokio-rs/axum/blob/main/examples/chat/src/main.rs
    pub websocket_tx: broadcast::Sender<String>, // we do not need the rx
    pub connected: AtomicI8,
}

impl Room {
    pub async fn get(&self) -> Vec<GameEvent> {
        println!("room.get entered");
        let messages = self.messages.read().await;
        messages.clone().into_iter().collect()
    }

    pub async fn insert(&self, message: Msg) {
        println!("room.insert before binding");
        let mut binding = self.messages.write().await;
        println!("room.insert after binding");
        match message {
            Msg::Game { msg } => {
                let msg_clone = msg.clone().data;
                binding.push(msg);
                println!("room.insert after push");
                let _ = self
                    .websocket_tx
                    .clone()
                    .send(serde_json::to_string(&msg_clone).unwrap());
                println!("room.insert after websocket_tx.send")
            }
            Msg::Internal { event } => (),
        }
    }

    pub fn add_user(&self) {
        self.connected.fetch_add(1, Relaxed);
    }

    pub fn remove_user(&self) {
        self.connected.fetch_sub(1, Relaxed);
    }
}

pub struct RoomConnector {
    pub state: Arc<DashMap<String, Room>>,
}

impl RoomConnector {
    pub fn new() -> Self {
        RoomConnector {
            state: Arc::new(DashMap::new()),
        }
    }

    pub fn addRoom(&self, game_id: String, current_level: String, path: String) {
        let rooms = self.state.clone();
        if (!rooms.contains_key(&game_id)) {
            let (tx, _rx) = broadcast::channel::<String>(150);
            self.state.clone().insert(
                game_id,
                Room {
                    messages: RwLock::new(Vec::new()),
                    current_level: current_level,
                    last_save: None,
                    path: path,
                    websocket_tx: tx,
                    connected: AtomicI8::new(0),
                },
            );
        };
    }

    pub async fn connect(&self, game_id: Arc<String>, socket: WebSocket, _user: &str) {
        // let game_id: Arc<&str> = Arc::new(game_id);

        if let Some(room) = self.state.clone().get(game_id.clone().as_ref()) {
            // TODO what about the case where the game doesn't exist?

            let (mut sender, mut receiver) = socket.split();

            let mut server_to_client_messages = room.websocket_tx.subscribe();
            std::mem::drop(room);

            let spawn_state = self.state.clone();
            let send_game_id: Arc<String> = game_id.clone();

            // these two take messages from the broadcast channel and sends it back to the client over their websocket
            let mut send_task = tokio::spawn(async move {
                // todo can probably chunk this
                let mut stream = stream::iter(
                    spawn_state
                        .get(send_game_id.as_ref())
                        .unwrap()
                        .get()
                        .await
                        .into_iter()
                        .map(|x| x.data),
                );
                println!("room.get exited into stream");
                // still have a race here, messages could be added while I'm streaming all these values but before the websocket gets connected.
                // maybe add some new sort of lock here like an atomic boolean or something? use a write lock?
                while let Some(value) = stream.next().await {
                    let msg: String = serde_json::to_string(&value).unwrap();
                    if sender.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
                println!("finished streaming back to the client");
                println!("hooking up websocket properly now");

                // connect the room broastcast to the client (recieve) websocket
                while let Ok(msg) = server_to_client_messages.recv().await {
                    // In any websocket error, break loop.
                    dbg!(msg.clone());
                    if sender.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            });

            let recv_state = self.state.clone();
            let recv_game_id: Arc<String> = game_id.clone();

            let mut recv_task = tokio::spawn(async move {
                while let Some(Ok(Message::Text(text))) = receiver.next().await {
                    let room = recv_state.get(recv_game_id.clone().as_str()).unwrap(); // what to do if this doens't exist
                    dbg!(text.clone());
                    match serde_json::from_str::<Event>(&text) {
                        Ok(v) => {
                            // I wonder if this should go in it's own thread?
                            // weird when I tried to spawn a thread to do this get tons
                            // of ownership issues on gtid and sender
                            let msg = Msg::Game {
                                msg: GameEvent {
                                    data: v,
                                    game_id: recv_game_id.clone().to_string(),
                                    user: None,
                                    level: None,
                                },
                            };

                            room.insert(msg).await;
                        }
                        Err(e) => {
                            dbg!(text);
                            tracing::error!(
                                "Something went wrong with {:?}, error: {:?}",
                                recv_game_id.clone(),
                                &e
                            )
                        }
                    }
                }
            });

            let error_game_id = game_id.clone();
            // If any one of the tasks run to completion, we abort the other.
            tokio::select! {
                _ = (&mut send_task) => {
                    // no way to block on this which is why we see a log message of 1
                    recv_task.abort();
                    tracing::info!("recv_task abort for game {:?} user {:?}", error_game_id.to_string(), &_user);
                }, // . here add a print statement, curuous if we can use this + receiver_count to clean out our map later
                _ = (&mut recv_task) => {
                    send_task.abort();
                    tracing::info!("send_task abort for game {:?} user {:?}", error_game_id.to_string(), &_user);
                }
            };

            // this takes messages from client and sends them to the server broadcast channel
        } else {
            let _ = socket.close();
        }
    }
}
