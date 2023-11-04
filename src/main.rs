mod domain;
mod state;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Form, Json, Path, Query, State,
    },
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
};
use dashmap::DashMap;

use crate::state::memory::VecState;
use domain::{
    game::{DTOState, GameState},
    message,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use state::memory::MemoryHandler;
use state::memory::MemoryReceiver;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    state_tx: Sender<message::Message>,
    game_state: Arc<MemoryHandler<VecState>>,
    tx: broadcast::Sender<String>,
}

#[derive(Deserialize)]
struct MapId {
    id: String,
}

#[derive(Deserialize)]
struct CreateGameParam {
    description: String,
    user: String,
    mode: String, // dungeon, world
    x: u32,
    y: u32,
}

#[derive(Deserialize)]
struct GameLevelIdParam {
    game_id: String,
    level_id: String,
}

impl CreateGameParam {
    fn validate(self) -> Result<CreateGameParam, String> {
        if self.x >= 0 && self.x <= 1000 && self.y >= 0 && self.y <= 1000 {
            Ok(self)
        } else {
            Err("Out of bounds".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_form=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our mpsc channel for processing messages
    let (state_tx, state_rx) = mpsc::channel::<message::Message>(100);

    // spawn a thread to listen

    let d: Arc<DashMap<String, VecState>> = Arc::new(DashMap::new());

    let memory_handler = MemoryHandler::make(d.clone());

    let mut memory_receiver = MemoryReceiver::make(state_rx, d.clone());

    tokio::spawn(async move {
        memory_receiver.start().await;
    });

    let (tx, _rx) = broadcast::channel(100);

    let state = AppState {
        state_tx: state_tx,
        game_state: Arc::new(memory_handler),
        tx: tx,
    };

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    // build our application with some routes
    let app = Router::new()
        .route("/", get(show_hello))
        // .route("/init", get(init_map)) // todo update this to take x,y params. ID will be generated server side.
        // .route("/load", get(load_map)) // remove this eventually once you get login flow working
        // .route("/load/:map_id", get(load_specific_map))
        .route("/save", post(save_game))
        .route("/create_game", post(create_game))
        .route("/load_game/:game_id", get(load_game))
        .route("/save_game_level", post(save_game_level))
        .with_state(state)
        .layer(cors);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct Hello {
    name: String,
}

async fn show_hello() -> Response {
    let hello = Hello {
        name: String::from("world"),
    };

    Json(hello).into_response()
}

// async fn init_map() -> Json<domain::Game::InitMap> {
//     let map = domain::Game::InitMap::default("SomeIdHere".to_string());
//     Json(map)
// }

async fn load_map() -> Json<VecState> {
    let gamestate = domain::game::GameState::make("Some Description".to_string(), (250, 250));
    Json(gamestate)
}

async fn load_game(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let zz = Arc::clone(&state.game_state);

    // todo: needs the specific level to load as well
    let resp = zz.get_game_json(&id, "dummy_value_fix_me").await;

    match resp {
        Some(game) => game.into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
async fn create_game(
    State(state): State<AppState>,
    Query(params): Query<CreateGameParam>,
) -> Response {
    let tt = state.game_state.clone();
    match params.validate() {
        Ok(params) => {
            let r = tokio::task::spawn(async move {
                tt.create_game(&params.description, &(params.x, params.y))
            })
            .await;
            match r {
                Ok(r) => Json(r).into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

async fn save_game(State(state): State<AppState>, Json(game): Json<DTOState>) -> Response {
    let s = state.clone();
    let game_id = game.id.clone();
    // TODO -> change this to be a direct save
    // probably still in it's own thread
    tokio::spawn(async move {
        s.state_tx
            .clone()
            .send(message::Message::EntireGame {
                game: game.toRust(),
            })
            .await
            .unwrap()
    })
    .await
    .unwrap();

    StatusCode::OK.into_response()
}

async fn save_game_level(
    State(state): State<AppState>,
    Json(game_lvl): Json<GameLevelIdParam>,
) -> Response {
    let s = state.clone();
    tokio::spawn(async move {
        let _ = s
            .state_tx
            .clone()
            .send(message::Message::TriggerSave {
                game_id: game_lvl.game_id,
                level_id: game_lvl.level_id,
            })
            .await;
    })
    .await;

    StatusCode::OK.into_response()
}

// Websocket stuff
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            // If username that is sent by client is not taken, fill username string.
            // check_username(&state, &mut username, &name);

            // // If not empty we want to quit the loop else we want to quit function.
            // if !username.is_empty() {
            //     break;
            // } else {
            //     // Only send our client that username is taken.
            //     let _ = sender
            //         .send(Message::Text(String::from("Username already taken.")))
            //         .await;

            //     return;
            // }
            println!("message!")
        }
    }

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

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
    let tx = state.tx.clone();
    let name = username.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let _ = tx.send(format!("{name}: {text}"));
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    let msg = format!("{username} left.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

    // Remove username from map so new clients can take it again.
    // state.user_set.lock().unwrap().remove(&username);
}

// fn check_username(state: &AppState, string: &mut String, name: &str) {
//     let mut user_set = state.user_set.lock().unwrap();

//     if !user_set.contains(name) {
//         user_set.insert(name.to_owned());

//         string.push_str(name);
//     }
// }
