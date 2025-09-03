use std::sync::Arc;

use sqlx::{types::Uuid, PgPool};
use validator::Validate;

use crate::domain::room::{Game, GameCreate, GameResponse};

pub struct GameService {
    db: Arc<PgPool>,
}

impl GameService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn create_game(&self, req: GameCreate) -> anyhow::Result<GameResponse> {
        req.validate()?;

        let game = sqlx::query_as!(
            Game,
            r#"
            INSERT INTO games(name, room_id)
            VALUES($1, $2)
            RETURNING id, name, room_id, created_at
            "#,
            req.name,
            req.room_id
        )
        .fetch_one(&*self.db)
        .await?;

        Ok(GameResponse::from(game))
    }

    pub async fn get_game(&self, id: Uuid) -> anyhow::Result<GameResponse> {
        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT id, name, room_id, created_at
            FROM games
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.db)
        .await?;

        Ok(GameResponse::from(game))
    }

    pub async fn list_games_in_room(&self, room_id: Uuid) -> anyhow::Result<Vec<GameResponse>> {
        let games = sqlx::query_as!(
            Game,
            r#"
            SELECT id, name, room_id, created_at
            FROM games
            WHERE room_id = $1
            ORDER BY created_at DESC
            "#,
            room_id
        )
        .fetch_all(&*self.db)
        .await?;

        Ok(games.into_iter().map(GameResponse::from).collect())
    }

    pub async fn delete_game(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM games
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.db)
        .await?;

        Ok(())
    }
}