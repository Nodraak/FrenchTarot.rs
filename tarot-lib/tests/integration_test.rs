use uuid::Uuid;

use tarot_lib::game::game;
use tarot_lib::game::events;
use tarot_lib::player::Player;


#[test]
fn main() {

    // p1 creates a new game

    let p1 = Player {
        uuid: Uuid::new_v4(),
        username: "p1".to_string(),
    };

    let mut game = game::GameState::new(3, &p1);

    // p2 joins

    let p2 = Player {
        uuid: Uuid::new_v4(),
        username: "p2".to_string(),
    };

    game.update(&events::Event::GameJoin(events::WsConnectPayload {
        uuid: p2.uuid,
        username: p2.username.clone(),
    }));

    assert_eq!(game.players_data.len(), 2);

    // p3 joins and game starts

    let p3 = Player {
        uuid: Uuid::new_v4(),
        username: "p3".to_string(),
    };

    game.update(&events::Event::GameJoin(events::WsConnectPayload {
        uuid: p3.uuid,
        username: p3.username.clone(),
    }));

    assert_eq!(game.players_data.len(), 3);
    // TODO assert state changed
}
