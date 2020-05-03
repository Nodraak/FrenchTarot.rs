use web_sys;

use tarot_lib::game::events_data::GameData;


pub fn events_append_str(msg: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let events = document.get_element_by_id("events").unwrap();
    let e = format!("<p>{}</p>", msg);
    events.set_inner_html(&(events.inner_html() + &e));
}

pub fn game_update(game_data: &GameData) {
    let game = &game_data.game;

    let document = web_sys::window().unwrap().document().unwrap();
    let main = document.get_element_by_id("game").unwrap();
    main.set_inner_html(&format!(
        r#"
            <ul>
                <li>Id: <a href="/game/play/{}">{}</a></li>
                <li>Players: {}/{}</li>
                <li>TODO... Status?</li>
            </ul>
        "#,
        game.uuid.to_string(), game.uuid.to_string(),
        game.players.len(), game.max_players_count,
    ));
}
