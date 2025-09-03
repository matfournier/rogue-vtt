use std::{sync::Arc, time::Duration};

use rand::Rng;
use sqlx::{types::Uuid, PgPool};
use validator::Validate;

use crate::domain::room::{Auth, Room, RoomCreate, RoomResponse};

pub struct RoomService {
    db: Arc<PgPool>,
}

impl RoomService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn create_room(&self, req: RoomCreate) -> anyhow::Result<RoomResponse> {
        req.validate()?;

        let password_hash = crate::util::password::hash(req.password).await?;

        let room = sqlx::query_as!(
            Room,
            r#"
            INSERT INTO rooms(name, password_hash)
            VALUES($1, $2)
            RETURNING id, name, password_hash, created_at
            "#,
            req.name,
            password_hash
        )
        .fetch_one(&*self.db)
        .await?;

        Ok(RoomResponse::from(room))
    }

    pub async fn get_room(&self, id: Uuid) -> anyhow::Result<RoomResponse> {
        let room = sqlx::query_as!(
            Room,
            r#"
            SELECT id, name, password_hash, created_at
            FROM rooms
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.db)
        .await?;

        Ok(RoomResponse::from(room))
    }

    pub async fn delete_room(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM rooms
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.db)
        .await?;

        Ok(())
    }

    pub async fn verify_room(&self, auth: &Auth) -> anyhow::Result<()> {
        let maybe_hash = sqlx::query!(r#"SELECT password_hash FROM rooms WHERE id = $1"#, auth.id)
            .fetch_optional(&*self.db)
            .await?;

        if let Some(row) = maybe_hash {
            let verified =
                crate::util::password::verify(auth.password.clone(), row.password_hash).await?;

            if verified {
                return Ok(());
            }
        }

        // Sleep a random amount of time to avoid leaking room existence via timing attacks
        let sleep_duration =
            rand::thread_rng().gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
        tokio::time::sleep(sleep_duration).await;

        Err(anyhow::anyhow!("Authentication failed"))
    }

    // You can add more methods as needed:
    // pub async fn list_games(&self) -> anyhow::Result<Vec<GameResponse>> { ... }
    // pub async fn update_game(&self, game_id: Uuid, req: GameUpdate) -> anyhow::Result<GameResponse> { ... }
}
