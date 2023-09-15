use crate::domain::Game::Level;
use crate::domain::Message::Message;
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::mpsc::Receiver;

pub struct MemoryHandler<T> {
    rx: Receiver<Message>,
    state: RwLock<HashMap<String, T>>,
}

impl<T> MemoryHandler<T> {
    pub fn make(rx: Receiver<Message>) -> Self {
        MemoryHandler {
            rx: rx,
            state: RwLock::new(HashMap::new()),
        }
    }
}

impl MemoryHandler<Level> {
    pub async fn start(&mut self) {
        while let Some(message) = self.rx.recv().await {
            println!("Recieved a save message!");
            match message {
                Message::EntireLevel { level: lvl } => {
                    let _ = dbg!(lvl.clone());
                    self.add(&lvl.id.clone(), lvl);
                }
                Message::TriggerSave => println!("triggered save"),
            };
        }
    }

    fn add(&mut self, key: &str, element: Level) {
        let mut w = self.state.write().unwrap();
        w.insert(key.to_string(), element);
    }
}
