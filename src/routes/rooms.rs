use std::time::Duration;

use crate::domain::room::{RoomAuth, RoomCreate};
use crate::routes::Result;
use anyhow::anyhow;
use axum::{http::StatusCode, Extension, Form};
use rand::Rng;
use sqlx::{PgExecutor, PgPool};
use validator::Validate;

// https://github.com/tokio-rs/axum/discussions/1830 for discussion State vs Extension
pub async fn create_room(db: Extension<PgPool>, Form(req): Form<RoomCreate>) -> Result<StatusCode> {
    req.validate()
        .map_err(|e| anyhow!(e).context("Failed to validate user"))?;

    let RoomCreate {
        game_name,
        password,
    } = req;

    let password_hash = crate::util::password::hash(password).await?;

    // todo return this
    let _game_id = sqlx::query_scalar!(
        r#"
        INSERT INTO rooms(game_name, password_hash)
        VALUES($1, $2)
        RETURNING game_id
        "#,
        game_name,
        password_hash
    )
    .fetch_one(&*db)
    .await;

    Ok(StatusCode::NO_CONTENT)
}

async fn verify_user(room: &RoomAuth, db: impl PgExecutor<'_> + Send) -> Result<bool> {
    let maybe_pw = sqlx::query!(
        r#"SELECT password_hash FROM rooms WHERE game_id = $1"#,
        room.room_id
    )
    .fetch_optional(db)
    .await?;

    if let Some(hash) = maybe_pw {
        let verified =
            crate::util::password::verify(room.password.clone(), hash.password_hash).await?;

        if verified {
            return Ok(true);
        }
    }

    // Sleep a random amount of time to avoid leaking existence of a user in timing.
    let sleep_duration =
        rand::thread_rng().gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
    tokio::time::sleep(sleep_duration).await;
    Err(super::HttpError::Unauthenticated)
}
