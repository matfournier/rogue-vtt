use crate::domain::Game::GameState;
use crate::domain::Message::Message;
use crate::domain::{self, Game};

use axum::Json;
use dashmap::DashMap;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;
// look at this example I think it makes more sense.

// use std::sync::{Arc, RwLock};
// use std::thread;
// use std::time::Duration;

// struct World {
//     players: Vec<RwLock<Player>>,
// }

// struct Player {
//     name: String,
//     score: usize,
// }

// fn main() {
//     let world = Arc::new(World {
//         players: vec![
//             RwLock::new(
//                 Player {
//                     name: "Hans".to_owned(),
//                     score: 1
//                 }
//             )
//         ]
//     });

//     let world_thread = world.clone();

//     thread::spawn(move || {
//         let mut lock = world_thread.players[0].write().unwrap();

//         lock.name = "Johann".to_owned();
//     });

//     thread::sleep(Duration::new(1, 0));

//     println!("{}", world.players[0].read().unwrap().name);
// }

// can you simiplify this down to just RwLock<HashMap<String, T>> ?

// https://stackoverflow.com/questions/31790411/how-to-get-mutable-struct-from-hashmap
// needs to be more complicated, can't ust be T
// probably needs a RefCell<T> or something, see the above post.

// Need to change this to store Arc<T> or Mutex<T> or something that let's me read a shared reference from another thread
// another option: make a method take a f to change the value entirely?

pub struct MemoryReceiver {
    state: Arc<DashMap<String, GameState>>,
    rx: Receiver<Message>,
}
impl MemoryReceiver {
    pub fn make(rx: Receiver<Message>, d: Arc<DashMap<String, GameState>>) -> Self {
        MemoryReceiver { state: d, rx: rx }
    }
}

impl MemoryReceiver {
    pub async fn start(&mut self) {
        while let Some(message) = self.rx.recv().await {
            println!("Recieved a save message!");
            match message {
                Message::EntireGame { game } => {
                    let _ = dbg!(game.clone());
                    let s = self.state.clone();
                    s.insert(game.game_id.clone(), game)

                    // self.add(&lvl.id.clone(), lvl);
                }
            };
        }
    }
}

pub struct MemoryHandler<T> {
    state: Arc<DashMap<String, T>>,
}

impl<T> MemoryHandler<T>
where
    T: Clone + Serialize,
{
    pub fn make(d: Arc<DashMap<String, T>>) -> Self {
        MemoryHandler { state: d }
    }
    fn add(&self, key: &str, element: T) {
        let w = self.state.clone();
        w.insert(key.to_string(), element);
    }

    // Only way to do this is to clone it
    // If you return a reference, the T can be updated at any time via the processing thread
    // so what would you be pointing to in memory?
    // async fn get(self, key: &str) -> Option<&T> {
    //     let w = self.state.read();
    //     w.await.get(key).map(|x| x.clone())
    //     // w.get(key).map(|x| Rc::new(x.clone()))
    // }

    fn get_json(&self, key: &str) -> Option<Json<T>> {
        let w = self.state.clone();
        w.get(key).map(|x| Json(x.clone()))
    }
}

impl MemoryHandler<GameState> {
    pub fn create_game(&self, description: &str, xy: &(i32, i32)) -> String {
        let gamestate = domain::Game::GameState::make(description.to_string(), *xy);
        // TODO: write to durable store here
        // .     also check if it already exists
        let game_id = &gamestate.game_id.clone();
        self.add(&gamestate.game_id.clone(), gamestate);
        dbg!(game_id.clone());
        game_id.to_string()
    }
    pub fn get_game_json(&self, id: &str) -> Option<Json<GameState>> {
        self.get_json(id)
    }
    // TODO: loading a level within an existing game
    // data repr. is all wrong for this.
}
