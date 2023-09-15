use crate::domain::Game::Level;
use crate::domain::Message::Message;
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;

pub struct MemoryHandler<T> {
    rx: Receiver<Message>,
    state: HashMap<String, T>,
}

impl<T> MemoryHandler<T> {
    fn make(rx: Receiver<Message>) -> Self {
        MemoryHandler {
            rx: rx,
            state: HashMap::new(),
        }
    }

    fn add(&mut self, key: &str, element: T) {
        self.state.insert(key.to_string(), element);
    }
}

impl MemoryHandler<Level> {
    async fn start(&mut self) {
        while let Some(message) = self.rx.recv().await {
            println!("Recieved a save message!");
            match message {
                Message::EntireLevel { level: lvl } => {
                    self.add(&lvl.id.clone(), lvl.clone());
                    let _ = dbg!(lvl.clone());
                }
                Message::TriggerSave => println!("triggered save"),
            };
        }
    }
}
