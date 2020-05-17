
// phase: enum: waiting_players, dispensing_cards, talking, playing, finished
// creator: player
// players: vec<player>[x]
// phaseData
    // waiting
        // waiting since
    // talking
        // best_talk: <player, what>

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::card::{Card, CardSuit};
use crate::game::events::Event;
use crate::player::Player;


//
// Client and Server data
//

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientGameState {
    pub game_data: GameState,
    pub player_data: PlayerState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerGameState {
    pub game_data: GameState,
    pub players_data: Vec<(Uuid, PlayerState)>,
}

// public data (shared among every player as it is)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    max_players: i8,                            // used to auto start game
    players_username: Vec<(Uuid, String)>,      // by seating order, counter-clockwise
    state: State,
    active_player: Option<Uuid>,                // who are we waiting for, if any

    // TODO leader?
    king: Option<CardSuit>,
}

// private data (player specific, details are hidden to non owner)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    player_uuid: Uuid,
    hand: Option<CardsPile>,
    dog: Option<CardsPile>,
    scoring_pile: Option<CardsPile>,
}

//
// Enum helpers
//

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    WaitingPlayers,     // game is created, waiting for players
    DealingCards,       // automated for now, might be manual later
    Bidding,            // players talk
    PreparingKing,      // leader calls the king
    PreparingDog,       // leader makes the dog
    Playing,            // main game
    Finished,           // end of game, showing score
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CardsPile {
    Visible(Vec<Card>),     // all cards
    Hidden(i8),             // only cards count
}

//
// Methods
//

impl GameState {
    pub fn new(max_players: i8, creator: &Player) -> Self {
        GameState {
            max_players: max_players,
            players_username: vec![
                (creator.uuid, creator.username.clone()),
            ],
            state: State::WaitingPlayers,

            // game is not started yet
            active_player: None,
            king: None,
        }
    }

    pub fn update(&mut self, event: &Event) -> Result<(), String> {

        Ok(())
    }
}

impl PlayerState {
    pub fn new(player: &Player) -> Self {
        PlayerState {
            player_uuid: player.uuid,

            // game is not started yet
            hand: None,
            dog: None,
            scoring_pile: None,
        }
    }
}
