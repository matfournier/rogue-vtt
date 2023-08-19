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
    http::Method,
    response::{Html, IntoResponse, Json, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_form=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with some routes
    let app = Router::new()
        .route("/", get(show_form).post(accept_form))
        .route("/hello", get(show_hello))
        .route("/init", get(init_map)) // todo update this to take x,y params. ID will be generated server side.
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
