
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

use crate::game::events::Event;
use crate::player::Player;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    // TODO: have a hasmap player_uuid->Player, and a creator_uuid

    pub uuid: Uuid,
    pub max_players_count: i32,
    pub players: Vec<Player>,
    pub creator_uuid: Uuid,

//    pub phase: GamePhase,
}


impl Game {
    pub fn update(&mut self, event: &Event) -> Result<(), String> {

        Ok(())
    }
}
