use crate::domain::game::DTOState;
use crate::domain::game::GameState;
use crate::domain::game::Tile;
use crate::VecState;
use anyhow::Result;
use roguevtt_server::configuration::DatabaseSettings;
use serde_json;
use sqlx::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

struct DB {
    pool: Pool<Postgres>,
}

impl DB {
    pub fn make(configuration: &DatabaseSettings) -> DB {
        let connection_pool = PgPool::connect_lazy(&configuration.connection_string())
            // .await
            .expect("Failed to connect to postgres");

        DB {
            pool: connection_pool,
        }
    }

    pub async fn save(self, gs: &GameState<Vec<Tile>>) -> Result<()> {
        // probaly want to deal with errors here
        let conn = self.pool.acquire().await?;
        let json: String = serde_json::to_string(gs)?;
        let level_id = Uuid::parse_str(&gs.level.id.to_string()).unwrap(); // fix;
        let game_id = Uuid::parse_str(&gs.id).unwrap(); // fix ;
        let description = &gs.level.description[..64]; // todo trim this to 64 characters
        let level_type = gs.level.kind.clone() as i16;

        // TODO
        // should do something with the actual result here
        // not just reutrn Ok(());
        sqlx::query!(
            r#"
        INSERT INTO levels(level_id, game_id, description, level_type, data) VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (level_id) DO UPDATE
         SET description = $3,
             data = $5
        "#,
            level_id,
            game_id,
            description,
            level_type,
            json
        ).execute(&self.pool).await?;
        Ok(())
    }
}

pub async fn save(gs: &VecState) {
    let game = gs.id.clone();
    let path = game_to_path(&game);

    // todo deal with this better.
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .await
        .unwrap();
    let _ = file.write_all(gs.to_json().as_bytes()).await;
}

pub fn game_level_to_path(game_id: &str, level_id: &str) -> String {
    let root = "./localdb";
    format!("{root}/{game_id}_{level_id}.json")
}

pub fn game_to_path(game_id: &str) -> String {
    let root = "./localdb";
    format!("{root}/{game_id}.json")
}

pub fn key_path(game_id: &str) -> String {
    let root = "./localdb";
    format!("{root}/{game_id}.key")
}

pub async fn exists(path: &str) -> bool {
    let attr = fs::metadata(path).await;
    match attr {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn game_exists(game_id: &str) -> bool {
    exists(&key_path(game_id)).await
}

// todo make this all safter.
pub async fn load_json(path: &str) -> Option<DTOState> {
    let is_present = exists(path).await;
    if is_present {
        let mut file = OpenOptions::new().read(true).open(path).await.unwrap();
        let mut buffer = Vec::new();
        // read the whole file
        file.read_to_end(&mut buffer).await.unwrap();
        let s = match std::str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let v: Option<DTOState> = serde_json::from_str(&s).ok();
        v
    } else {
        None
    }
}

pub async fn load_rust(path: &str) -> Option<VecState> {
    let maybe_state = load_json(path).await;
    maybe_state.map(|gs| gs.to_rust())
}

pub async fn save_key(game_id: &str, key: &str) {
    let path = key_path(game_id);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .await
        .unwrap();
    let _ = file.write_all(key.to_string().as_bytes()).await;
}

pub async fn check_key(game_id: &str, key: &str) -> bool {
    let path = key_path(game_id);
    let is_present = exists(&path).await;
    if is_present {
        let mut file = OpenOptions::new().read(true).open(path).await.unwrap();
        let mut buffer = Vec::new();
        // read the whole file
        file.read_to_end(&mut buffer).await.unwrap();
        let s = match std::str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        s == key
    } else {
        false
    }
}
