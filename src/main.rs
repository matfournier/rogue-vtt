mod domain;
mod state;

use crate::state::loader;
use crate::state::loader::Loader;
use crate::state::loader::LocalLoader;
use crate::state::memory::GameChannel;
use axum::routing::get_service;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Json, Path, Query, State,
    },
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
};
use dashmap::DashMap;
use env_logger::Env;
use roguevtt_server::configuration::get_configuration;
use sqlx::PgPool;
use state::memory::SocketConnector;
use std::time::Duration;
use tokio::{task, time};
use tower_http::compression::CompressionLayer;

use crate::state::memory::VecState;
use domain::event;
use serde::{Deserialize, Serialize};
use state::memory::Dispatcher;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
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
    x: i16,
    y: i16,
    pw: String,
}

#[derive(Deserialize)]

struct LoadGameParam {
    pw: String,
}

#[derive(Deserialize)]
struct GameLevelIdParam {
    game_id: String,
    level_id: String,
    pw: String,
}

impl CreateGameParam {
    fn validate(self) -> Result<CreateGameParam, String> {
        if self.x <= 1000 && self.y <= 1000 {
            Ok(self)
        } else {
            Err("Out of bounds".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "example_form=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration.yaml");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");

    // build our mpsc channel for processing messages
    let (state_tx, state_rx) = mpsc::channel::<event::Msg>(1000);

    // spawn a thread to listen

    let loader: Arc<dyn Loader + Send + Sync> = Arc::new(LocalLoader);

    let state: Arc<DashMap<String, GameChannel>> = Arc::new(DashMap::new());
    let mut dispatcher = Dispatcher::make(state_rx, loader.clone(), state.clone());

    // https://stackoverflow.com/questions/76015781/could-not-prove-that-closure-is-send maybe

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
        .route("/create_game", post(create_game))
        .route("/load_game/:game_id", get(load_game))
        .route("/websocket", get(websocket_handler))
        .fallback(
            get_service(ServeDir::new("./client/dist")).handle_error(|_| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }),
        )
        .with_state(state)
        .layer(CompressionLayer::new())
        .layer(cors);

    let _ = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(60000));
        loop {
            interval.tick().await;
            let _ = arc_state_tx
                .clone()
                .send(event::Msg::Internal {
                    event: event::InternalEvent::Persist,
                })
                .await;
        }
    });

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn load_game(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(pw): Query<LoadGameParam>,
) -> Response {
    let loader = state.loader.clone();
    let allowed = loader.check_key(&id, &pw.pw).await;
    if allowed {
        let resp = loader.get_for_json(loader::path_with_game(&id)).await;
        match resp {
            Some(game) => serde_json::to_string(&game).unwrap().into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        }
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
async fn create_game(
    State(state): State<AppState>,
    Query(params): Query<CreateGameParam>,
) -> Response {
    let loader = state.loader.clone();
    match params.validate() {
        Ok(params) => {
            let game_state =
                domain::game::GameState::make(params.description.to_string(), (params.x, params.y));

            loader.save_direct(&game_state).await;
            loader.save_key(&game_state.id, &params.pw).await;

            tracing::info!("created game {:?}", &game_state.id);
            game_state.to_json().into_response()
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
        .check_key(&params.game_id, &params.pw)
        .await;
    if exists {
        ws.on_upgrade(|socket| handle_socket(socket, state, params))
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

async fn handle_socket(ws: WebSocket, state: AppState, params: GameLevelIdParam) {
    state
        .connector
        .clone()
        .connect(&params.game_id, ws, "todo")
        .await
}
