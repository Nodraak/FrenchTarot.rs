use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::card::{Card, CardSuit};
use crate::game::game;


//
// Events
//

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    // websocket events
    WsConnect(WsConnectPayload),
    WsDisconnect(WsConnectPayload),

    // game data
    Game(GamePayload),

    // TODO heartbeat? with game state and active player

    // register as player
    GameJoin(WsConnectPayload),                    // on last player: transition WaitingPlayers -> DealingCards

    // TODO: deal cards manually?
    DealResult(DealResultPayload),                 // transition DealingCards -> Bidding

    // bids
    BidAnnounce(BidAnnouncePayload),
    BidResult(BidAnnouncePayload),                 // transition Bidding -> PreparingKing

    // preparing
    KingCalled(KingCalledPayload),                 // transition PreparingKing -> PreparingDog
    DogResult(DogResultPayload),                   // transition PreparingDog -> Playing

    PlayCard(PlayCardPayload),                     // on last card: transition Playing -> Finished

    // TODO event scores

/*
Might implement later:
    GameQuit,   // reason: rage_quit
    GameStart,  // TODO: reason: Complete / Majority / Master
*/
}

//
// Events payload
//

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsConnectPayload {
    pub uuid: Uuid,
    pub username: String,
}

pub type GamePayload = game::GameState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DealResultPayload {
    pub hand: Vec<Card>,
    pub dog: Vec<Card>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BidAnnouncePayload {
    pub player: Uuid,
    // TODO enum bid: petite, garde, ...
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KingCalledPayload {
    pub suit: CardSuit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DogResultPayload {
    pub dog: Option<Vec<Card>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayCardPayload {
    pub player: Uuid,
    pub card: Card,
}
