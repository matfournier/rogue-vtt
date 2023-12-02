// use dashmap::DashMap;
// use std::sync::Arc;
// use tokio::sync::broadcast;

// // should we have a channel to this dispatcher that handles messages from all websockets?
// // should the Memory state write to memory and then write to this?
// // . or should we spawn a thread to this and spawn a thread to the other one?
// //
// // I think it should be: channel to dispatcher, all websockets send to channel
// // . will need an "add new channel" message somehow
// // in the websocket handler it will be: send msg to state (thread 1) + send message to dispatcher (thread 2)

// pub struct Dispatcher {
//     // does the Sender need to be in an ARC?
//     // what happens when there are no more connections for this room, how do we remove this from the map ?
//     //   return the String to delete it from the Map maybe?
//     channels: Arc<DashMap<String, broadcast::Sender<String>>>,
// }

// impl Dispatcher {
//     fn new() -> Dispatcher {
//         let channels: Arc<DashMap<String, broadcast::Sender<String>>> = Arc::new(DashMap::new());
//         Dispatcher { channels: channels }
//     }

//     // todo need to check if it exists in the map already
//     // todo need to check to make sure the level is loaded into memory: if it isn't, fail.
//     fn add(id: String) {}
// }
