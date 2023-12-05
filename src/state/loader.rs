use crate::domain::game::GameState;
use crate::domain::game::Tile;
use async_trait::async_trait;

#[async_trait]
pub trait Loader {
    async fn get_for_memory(&self, path: String) -> Option<GameState<Vec<Option<u8>>>>;
    async fn get_for_json(&self, path: String) -> Option<GameState<Vec<Tile>>>;
    async fn save_direct(&self, state: &GameState<Vec<Option<u8>>>);
    async fn exists(&self, path: &str) -> bool;
}

pub struct LocalLoader;

#[async_trait]
impl Loader for LocalLoader {
    async fn get_for_memory(&self, path: String) -> Option<GameState<Vec<Option<u8>>>> {
        crate::state::db::load_rust(&path).await
    }
    async fn get_for_json(&self, path: String) -> Option<GameState<Vec<Tile>>> {
        crate::state::db::load_json(&path).await
    }
    async fn save_direct(&self, state: &GameState<Vec<Option<u8>>>) {
        crate::state::db::save(state).await
        // probably need some state passed in to make it make sense
        // fn will load the json into memory, apply the queue, then save.
        // ();
    }

    async fn exists(&self, path: &str) -> bool {
        crate::state::db::exists(&path).await
    }
}

pub fn path_with_game(game_id: &str) -> String {
    crate::state::db::game_to_path(game_id)
}
pub fn path_with_level(game_id: &str, level_id: &str) -> String {
    crate::state::db::game_level_to_path(game_id, level_id)
}
