use serde::{Serialize, Deserialize};

//use crate::player::Player;
use crate::game::game::Game;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsConnectData {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateGameData {
    pub game: Game,
    // possibly other fields
}

/*
#[derive(Clone, Serialize, Deserialize)]
pub struct DealResultData {
    // TODO hand
}
*/
