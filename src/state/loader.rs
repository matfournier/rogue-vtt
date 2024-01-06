use crate::domain::game::GameState;
use crate::domain::game::Tile;
use async_trait::async_trait;

#[async_trait]
pub trait Loader {
    async fn get_for_memory(&self, path: String) -> Option<GameState<Vec<Option<u16>>>>;
    async fn get_for_json(&self, path: String) -> Option<GameState<Vec<Tile>>>;
    async fn save_direct(&self, state: &GameState<Vec<Option<u16>>>);
    async fn exists(&self, path: &str) -> bool;
    async fn game_exists(&self, game_id: &str) -> bool;
    async fn save_key(&self, game_id: &str, key: &str);
    async fn check_key(&self, game_id: &str, key: &str) -> bool;
}

pub struct LocalLoader;

#[async_trait]
impl Loader for LocalLoader {
    async fn get_for_memory(&self, path: String) -> Option<GameState<Vec<Option<u16>>>> {
        crate::state::db::load_rust(&path).await
    }
    async fn get_for_json(&self, path: String) -> Option<GameState<Vec<Tile>>> {
        crate::state::db::load_json(&path).await
    }
    async fn save_direct(&self, state: &GameState<Vec<Option<u16>>>) {
        crate::state::db::save(state).await
        // probably need some state passed in to make it make sense
        // fn will load the json into memory, apply the queue, then save.
        // ();
    }

    async fn exists(&self, path: &str) -> bool {
        crate::state::db::exists(&path).await
    }

    async fn game_exists(&self, game_id: &str) -> bool {
        crate::state::db::game_exists(game_id).await
    }

    async fn save_key(&self, game_id: &str, key: &str) {
        crate::state::db::save_key(game_id, key).await
    }

    async fn check_key(&self, game_id: &str, key: &str) -> bool {
        crate::state::db::check_key(game_id, key).await
    }
}

pub fn path_with_game(game_id: &str) -> String {
    crate::state::db::game_to_path(game_id)
}
pub fn path_with_level(game_id: &str, level_id: &str) -> String {
    crate::state::db::game_level_to_path(game_id, level_id)
}
