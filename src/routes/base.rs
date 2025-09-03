use askama::Template;
use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::Askama;

pub async fn health() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}

pub async fn home() -> impl IntoResponse {
    Askama(IndexTemplate {
        title: "RogueVTT".to_string(),
    })
}

// figure out a better place for this
//

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
}
