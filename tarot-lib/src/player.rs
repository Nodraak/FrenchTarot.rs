// absolute
    // name: str
    // score
// phaseData:
    // playing
        // role: leader, follower, opponent
        // deck: vec<card>[]
        // heap: vec<card>[] -> cards won

use std::cmp::PartialEq;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Player {
    pub uuid: Uuid,
    pub username: String,
}
