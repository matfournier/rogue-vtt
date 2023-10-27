use super::game::GameState;

// idea that everything would get sent through the pipe to the handler
// can we send TriggerSave there too? Or should that be inside of the hanlder itself (probably?)
#[derive(Debug)]
pub enum Message {
    EntireGame { game: GameState },
    TriggerSave { game_id: String, level_id: String },
}
