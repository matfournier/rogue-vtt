//! Run with
//!
//! ```not_rust
//! cargo run -p example-form
//! ```

mod domain;

// mod prelude {
//     pub use crate::domain::Player;
// }

use axum::{
    extract::Form,
    extract::Json,
    extract::State,
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
};
use domain::messages::Level;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct HandlerState {
    tx: Sender<Level>,
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
    let (tx, rx) = mpsc::channel::<Level>(100);

    // spawn a thread to listen

    tokio::spawn(async move {
        consume_channel(rx).await;
    });

    let state = HandlerState { tx: tx };

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    // build our application with some routes
    let app = Router::new()
        .route("/", get(show_form).post(accept_form))
        .route("/hello", get(show_hello))
        .route("/init", get(init_map)) // todo update this to take x,y params. ID will be generated server side.
        .route("/load", get(load_map))
        .route("/save", post(save_map))
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

async fn init_map() -> Json<domain::messages::InitMap> {
    let map = domain::messages::InitMap::default("SomeIdHere".to_string());
    Json(map)
}

async fn load_map() -> Json<domain::messages::GameState> {
    let gamestate = domain::messages::GameState::make("Some Description".to_string(), (250, 250));
    Json(gamestate)
}
async fn save_map(State(state): State<HandlerState>, Json(level): Json<Level>) -> Response {
    tokio::spawn(async move {
        let _ = state.tx.clone().send(level).await;
    });
    StatusCode::OK.into_response()
}

async fn consume_channel(mut rx: Receiver<Level>) {
    while let Some(level) = rx.recv().await {
        println!("Recieved a save message!");
        dbg!(level);
    }
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    name: String,
    email: String,
}

async fn accept_form(Form(input): Form<Input>) {
    dbg!(&input);
}

//use axum::{
//    extract,
//    routing::post,
//    Router,
//};
//use serde::Deserialize;

//#[derive(Deserialize)]
//struct CreateUser {
//    email: String,
//    password: String,
//}
//
//async fn create_user(extract::Json(payload): extract::Json<CreateUser>) {
//    // payload is a `CreateUser`
//}

//let app = Router::new().route("/users", post(create_user));
