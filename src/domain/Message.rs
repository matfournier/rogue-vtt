use super::Game::{GameState, Level};

// idea that everything would get sent through the pipe to the handler
// can we send TriggerSave there too? Or should that be inside of the hanlder itself (probably?)

pub enum Message {
    // EntireLevel { level: Level },
    EntireGame { game: GameState },
    // TriggerSave, // probably don't need this.
}
