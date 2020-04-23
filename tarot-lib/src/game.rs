
// phase: enum: waiting_players, dispensing_cards, talking, playing, finished
// creator: player
// players: vec<player>[x]
// phaseData
    // waiting
        // waiting since
    // talking
        // best_talk: <player, what>

use uuid::Uuid;

use crate::player::Player;

pub struct Game<'a> {
    pub id: Uuid,
    pub creator: &'a Player,
    pub players: [&'a Player; 5],
}
