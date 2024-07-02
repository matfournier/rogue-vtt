mod domain;
mod state;

use crate::state::loader;
use crate::state::loader::Loader;
use crate::state::loader::LocalLoader;
use axum::routing::get_service;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Json, Path, Query, State,
    },
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
};
use lazy_static::lazy_static;
use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use roguevtt_server::configuration::get_configuration;
use roguevtt_server::configuration::SaveSettings;
use sqlx::PgPool;
use state::room::RoomConnector;
use std::str::FromStr;
use std::time::Duration;
use tokio::{task, time};
use tower_http::{
    compression::Compression,
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use uuid::Uuid;

use crate::state::memory::VecState;
use domain::event;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use tracing::Level;

#[derive(Clone)]
struct AppState {
    // state_tx: Arc<Sender<event::Msg>>,
    // connector: Arc<SocketConnector>,
    loader: Arc<dyn Loader + Send + Sync>,
    rooms: Arc<RoomConnector>,
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
}

#[derive(Deserialize)]
// struct LoadGameParam {
//     pw: String,
// }
// #[derive(Deserialize)]
struct GameLevelIdParam {
    game_id: String,
    level_id: String,
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

pub const TEMPLATE_PATH: &str = "app/templates";

lazy_static! {
    pub static ref TEMPLATES: AutoReloader = {
        AutoReloader::new(|notifier| {
            let mut env = Environment::new();
            env.set_loader(path_loader(TEMPLATE_PATH));
            notifier.watch_path(TEMPLATE_PATH, true);
            Ok(env)
        })
    };
}

#[tokio::main]
#[warn(clippy::future_not_send)]
async fn main() {
    let static_files = Compression::new(
        ServeDir::new("app") // if no files found, check your pwd to make sure it's at project root
            .fallback(ServeFile::new("app/404.html")),
    );
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("roguevtt".into(), std::io::stdout);

    let configuration = get_configuration().expect("Failed to read configuration.yaml");
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");

    let loader: Arc<dyn Loader + Send + Sync> = Arc::new(LocalLoader);

    let rooms = Arc::new(RoomConnector::new());

    let state = AppState {
        loader: loader.clone(),
        rooms: rooms.clone(),
    };

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    // build our application with some routes
    let app = Router::new()
        .route("/", get(home))
        .route("/create_game", post(create_game))
        .route("/load_game/:game_id", get(load_game))
        .route("/websocket", get(websocket_handler))
        .fallback_service(static_files)
        .with_state(state)
        .layer(CompressionLayer::new())
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    let _ = task::spawn(async move {
        save_all_games(rooms.clone(), configuration.saving).await;
    });

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn home() -> axum::response::Result<Html<String>> {
    let env = TEMPLATES.acquire_env().unwrap();
    let template = env.get_template("index.html").unwrap();
    let html = template
        .render(())
        .map_err(|_| StatusCode::from_u16(500).unwrap())?;
    Ok(html.into())
}

async fn save_all_games(rooms: Arc<RoomConnector>, settings: SaveSettings) {
    let mut interval = time::interval(Duration::from_millis(settings.period));
    loop {
        interval.tick().await;
        rooms.save_all(settings.parallelism).await;
    }
}
async fn load_game(
    State(state): State<AppState>,
    Path(id): Path<String>,
    // Query(pw): Query<LoadGameParam>,
) -> Response {
    let loader = state.loader.clone();
    // let allowed = loader.check_key(&id, &pw.pw).await;
    let allowed = true;
    if allowed {
        let resp = loader.get_for_json(loader::path_with_game(&id)).await;
        match resp {
            Some(game) => {
                let rooms = state.rooms.clone();
                rooms.add_room(game.meta.id.clone(), game.level.id);

                tracing::info!("loading game {:?}", &game.meta.id);
                serde_json::to_string(&game).unwrap().into_response()
            }
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
            // loader.save_key(&game_state.id, &params.pw).await;

            tracing::info!("created game {:?}", &game_state.meta.id);
            let rooms = state.rooms.clone();
            rooms.add_room(game_state.meta.id.clone(), game_state.level.id);
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
    // let exists = state
    //     .loader
    //     .clone()
    //     .check_key(&params.game_id, &params.pw)
    //     .await;
    let exists = true;
    if exists {
        match Uuid::from_str(&params.game_id) {
            Ok(uuid) => {
                ws.on_upgrade(move |socket| handle_socket_room(socket, state, params, uuid))
            }
            Err(_) => StatusCode::NOT_FOUND.into_response(),
        }
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

async fn handle_socket_room(ws: WebSocket, state: AppState, params: GameLevelIdParam, uuid: Uuid) {
    println!("entered handle_socket_room");
    state
        .rooms
        .clone()
        .connect(Arc::new(uuid), ws, "todo")
        .await;
    println!("exited handle_socket_room");
}
