use super::Game::Level;

// idea that everything would get sent through the pipe to the handler
// can we send TriggerSave there too? Or should that be inside of the hanlder itself (probably?)

pub enum Message {
    EntireLevel { level: Level },
    TriggerSave, // probably don't need this.
}
