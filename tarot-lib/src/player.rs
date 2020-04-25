// absolute
    // name: str
    // score
// phaseData:
    // playing
        // role: leader, follower, opponent
        // deck: vec<card>[]
        // heap: vec<card>[] -> cards won

use uuid::Uuid;

pub struct Player {
    pub uuid: Uuid,
    pub username: String,
}
