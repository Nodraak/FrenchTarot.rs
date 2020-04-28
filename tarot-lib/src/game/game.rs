
// phase: enum: waiting_players, dispensing_cards, talking, playing, finished
// creator: player
// players: vec<player>[x]
// phaseData
    // waiting
        // waiting since
    // talking
        // best_talk: <player, what>

use serde::ser::{Serialize, Serializer, SerializeStruct};
use uuid::Uuid;

use crate::player::Player;


pub struct Game<'a> {
    pub uuid: Uuid,
    pub max_players_count: i32,
    pub players: Vec<Player>,
    pub creator: Option<&'a Player>,

//    pub phase: GamePhase,
}


impl Serialize for Game<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 4 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Game", 4)?;
        state.serialize_field("uuid", &self.uuid)?;
        state.serialize_field("max_players_count", &self.max_players_count)?;
        state.serialize_field("creator", &self.creator)?;
        state.serialize_field("players", &self.players)?;
        state.end()
    }
}
