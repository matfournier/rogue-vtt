use crate::domain::game::DTOState;
use crate::VecState;
use serde_json;
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
    file.write_all(gs.to_json().as_bytes()).await;
}

pub fn game_level_to_path(game_id: &str, level_id: &str) -> String {
    let root = "./localdb";
    format!("{root}/{game_id}_{level_id}.json")
}

pub fn game_to_path(game_id: &str) -> String {
    let root = "./localdb";
    format!("{root}/{game_id}.json")
}

pub async fn exists(path: &str) -> bool {
    let attr = fs::metadata(path).await;
    match attr {
        Ok(_) => true,
        Err(_) => false,
    }
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
