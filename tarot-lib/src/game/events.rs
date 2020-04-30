use serde::{Serialize, Deserialize};

use crate::game::events_data;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    WsConnect(events_data::WsConnectData),
    WsDisconnect(events_data::WsConnectData),

    Game(events_data::GameData),

    GameJoin(events_data::WsConnectData),

/*
    GameQuit, // reason: rage_quit

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


/*
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameState {
    WaitingPlayers,
    DealingCards,
    Bidding,
    Preparing,
    Playing,
    Finished,
}
*/
