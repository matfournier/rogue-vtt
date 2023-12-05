mod domain;
mod state;

use crate::state::loader;
use crate::state::loader::Loader;
use crate::state::loader::LocalLoader;
use crate::state::memory::GameChannel;

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
use state::memory::SocketConnector;
use tower_http::compression::CompressionLayer;

use crate::state::memory::VecState;
use domain::{
    event,
    game::{DTOState, GameState},
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
// use state::memory::MemoryHandler;
// use state::memory::MemoryReceiver;
use state::memory::Dispatcher;
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
    state_tx: Arc<Sender<event::Msg>>,
    connector: Arc<SocketConnector>,
    loader: Arc<dyn Loader + Send + Sync>,
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
    let (state_tx, state_rx) = mpsc::channel::<event::Msg>(1000);

    // spawn a thread to listen

    let loader: Arc<dyn Loader + Send + Sync> = Arc::new(LocalLoader);

    let state: Arc<DashMap<String, GameChannel>> = Arc::new(DashMap::new());
    let mut dispatcher = Dispatcher::make(state_rx, loader.clone(), state.clone());

    let _ = tokio::spawn(async move {
        dispatcher.start().await;
    });

    let arc_state_tx = Arc::new(state_tx);

    let state = AppState {
        state_tx: arc_state_tx.clone(),
        connector: Arc::new(SocketConnector {
            state: state.clone(),
            loader: loader.clone(),
            sender: arc_state_tx.clone(),
        }),
        loader: loader.clone(),
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
        // .route("/save", post(save_game))
        .route("/create_game", post(create_game))
        .route("/load_game/:game_id", get(load_game))
        // .route("/save_game_level", post(save_game_level))
        .route("/websocket", get(websocket_handler))
        .with_state(state)
        .layer(CompressionLayer::new())
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

async fn load_game(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let loader = state.loader.clone();
    let resp = loader.get_for_json(loader::path_with_game(&id)).await;
    match resp {
        Some(game) => serde_json::to_string(&game).unwrap().into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
async fn create_game(
    State(state): State<AppState>,
    Query(params): Query<CreateGameParam>,
) -> Response {
    let loader = state.loader.clone();
    match params.validate() {
        Ok(params) => {
            // create & save the game
            // add to the Dispatcher

            //     pub fn create_game(&self, description: &str, xy: &(u32, u32)) -> VecState {
            //         let gamestate = domain::game::GameState::make(description.to_string(), *xy);
            //         // TODO: write to durable store here
            //         // .     also check if it already exists
            //         let game_id = &gamestate.id.clone();
            //         self.add(&gamestate.id.clone(), gamestate.clone());
            //         dbg!(game_id.clone());
            //         gamestate
            //     }
            let game_state =
                domain::game::GameState::make(params.description.to_string(), (params.x, params.y));

            loader.save_direct(&game_state).await;

            game_state.toJson().into_response()

            // let r = tokio::task::spawn(async move {
            //     tt.create_game(&params.description, &(params.x, params.y))
            // })
            // .await;
            // match r {
            //     Ok(r) => Json(r).into_response(),
            //     Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            // }
        }
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

// Websocket stuff: note needs to get the room from the gameId
// if the game doesn't exist then reject, should only connect valid games
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<GameLevelIdParam>,
) -> impl IntoResponse {
    let exists = state
        .loader
        .clone()
        .exists(&loader::path_with_game(&params.game_id))
        .await;
    if exists {
        ws.on_upgrade(|socket| handle_socket(socket, state, params))
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn handle_socket(ws: WebSocket, state: AppState, params: GameLevelIdParam) {
    state
        .connector
        .clone()
        .connect(&params.game_id, ws, "todo")
        .await
}
