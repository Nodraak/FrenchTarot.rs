use serde::{Serialize, Deserialize};

//use crate::player::Player;

// Clone, Debug, Serialize, Deserialize

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsConnectData {
    pub username: String,
}

/*
#[derive(Clone, Serialize, Deserialize)]
pub struct GameJoinData {
    pub username: String,
    // TODO: Player / Watcher
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DealResultData {
    // TODO hand
}
*/
