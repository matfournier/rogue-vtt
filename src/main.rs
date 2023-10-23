mod domain;
mod state;

use axum::{
    extract::Form,
    extract::Json,
    extract::Path,
    extract::Query,
    extract::State,
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
};
use dashmap::DashMap;
use domain::Game::{GameState, Id, Level};
use domain::Message;
use serde::{Deserialize, Serialize};
use state::Memory::MemoryHandler;
use state::Memory::MemoryReceiver;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct HandlerState {
    tx: Sender<Message::Message>,
    game_state: Arc<MemoryHandler<GameState>>,
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
    x: i32,
    y: i32,
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
    let (tx, rx) = mpsc::channel::<Message::Message>(100);

    // spawn a thread to listen

    let d: Arc<DashMap<String, GameState>> = Arc::new(DashMap::new());

    let memory_handler = MemoryHandler::make(d.clone());

    let mut memory_receiver = MemoryReceiver::make(rx, d.clone());

    tokio::spawn(async move {
        memory_receiver.start().await;
    });

    let state = HandlerState {
        tx: tx,
        game_state: Arc::new(memory_handler),
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

async fn load_map() -> Json<domain::Game::GameState> {
    let gamestate = domain::Game::GameState::make("Some Description".to_string(), (250, 250));
    Json(gamestate)
}

async fn load_game(State(state): State<HandlerState>, Path(id): Path<String>) -> Response {
    let zz = Arc::clone(&state.game_state);
    let resp = zz.get_game_json(&id);

    match resp {
        Some(game) => game.into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
async fn create_game(
    State(state): State<HandlerState>,
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

async fn save_game(State(state): State<HandlerState>, Json(game): Json<GameState>) -> Response {
    let s = state.clone();
    // TODO -> change this to be a direct save
    // probably still in it's own thread
    tokio::spawn(async move {
        let _ =
            s.tx.clone()
                .send(Message::Message::EntireGame { game: game })
                .await;
    });
    StatusCode::OK.into_response()
}
