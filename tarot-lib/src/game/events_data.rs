use serde::{Serialize, Deserialize};

//use crate::player::Player;
use crate::game::game::Game;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsConnectData {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameData {
    pub game: Game,
    // possibly other fields
}
