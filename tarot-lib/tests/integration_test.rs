use uuid::Uuid;

use tarot_lib::game::game::Game;
use tarot_lib::player::Player;


#[test]
fn main() {
    let p1 = Player {
        uuid: Uuid::new_v4(),
        username: "p1".to_string(),
    };
    let p2 = Player {
        uuid: Uuid::new_v4(),
        username: "p2".to_string(),
    };
    let p3 = Player {
        uuid: Uuid::new_v4(),
        username: "p3".to_string(),
    };
    let p4 = Player {
        uuid: Uuid::new_v4(),
        username: "p4".to_string(),
    };

    let g = Game {
        uuid: Uuid::new_v4(),
        max_players_count: 3,
        players: vec![p1, p2, p3],
        creator: Some(&p4),
    };
}
