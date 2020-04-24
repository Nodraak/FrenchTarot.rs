
// phase: enum: waiting_players, dispensing_cards, talking, playing, finished
// creator: player
// players: vec<player>[x]
// phaseData
    // waiting
        // waiting since
    // talking
        // best_talk: <player, what>

use super::player::Player;

pub struct Game<'a> {
    pub pk: i32,
    pub max_players_count: i32,
    pub creator: Option<&'a Player>,
    pub players: Option<[&'a Player; 5]>,
}
