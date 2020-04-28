use serde::{Serialize, Deserialize};

use crate::game::events::data::WsConnectData;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    WsConnect(WsConnectData),
    WsDisconnect(WsConnectData),

/*
    GameJoin(GameJoinData),
    GameQuit(GameJoinData),

    GameStart,  // TODO: reason: Complete / Majority / Master

    // TODO: deal cards manually?
    DealResult(DealResultData),

    BidAnnounce,
    BidResult,

    DogMade,
    KingCalled {c: Color},

    PlayCard {c: Card},
*/
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameState {
    WaitingPlayers,
    DealingCards,
    Bidding,
    Preparing,
    Playing,
    Finished,
}
