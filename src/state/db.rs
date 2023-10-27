use serde_json;
use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::domain::game::GameState;

pub async fn save(gs: &GameState) {
    let root = "./localdb";
    let game = gs.id.clone();
    let level = gs.level.id.clone();
    // let path = format!("{root}/{game}_{level}.json");
    let path = format!("{root}/{game}.json");

    let conents = serde_json::to_string(gs).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .await
        .unwrap();
    file.write_all(conents.as_bytes()).await;
}

pub async fn load(game_id: &str, level_id: &str) -> Option<GameState> {
    let root = "./localdb";
    // let path = format!("{root}/{game_id}_{level_id}.json");
    // until we get the level stuff sorted out
    let path = format!("{root}/{game_id}.json");
    let is_present = Path::new(&path.clone()).exists();
    if is_present {
        let mut file = OpenOptions::new().read(true).open(path).await.unwrap();
        let mut buffer = Vec::new();
        // read the whole file
        file.read_to_end(&mut buffer).await.unwrap();
        let s = match std::str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        Some(serde_json::from_str(&s).unwrap())
    } else {
        None
    }
}
