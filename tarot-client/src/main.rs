use uuid::Uuid;
use web_sys;
use wasm_bindgen::prelude::*;

mod conf {
    // Warning: by default, Rocket binds to ipv6, not ipv4
    pub static WS_ADDR: &str = "localhost:8001";
}
mod js_api;
mod ui;
mod utils;
mod websockets;


fn wrapped_main(document: &web_sys::Document, game_uuid: Uuid) -> utils::Result<()> {
    ui::table::init(&document, game_uuid)?;

    Ok(())
}


#[wasm_bindgen]
pub fn main(game_uuid_: String, player_uuid_: String) -> utils::Result<()> {
    let game_uuid = Uuid::parse_str(&game_uuid_).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    document.query_selector("#main > p").unwrap().unwrap().set_inner_html("Starting game... (2/3)");

    let r = wrapped_main(&document, game_uuid);

    if let Err(_) = r {
        let main = document.get_element_by_id("main").unwrap();
        main.set_inner_html(r#"
            <p>Error init ui</p>
        "#);
    }

    let ws_path = format!("/game/play/{}/{}", game_uuid_, player_uuid_);

    let r = websockets::client::main(format!("ws://{}{}", conf::WS_ADDR, ws_path));
    if let Err(_) = r {
        let main = document.get_element_by_id("main").unwrap();
        main.set_inner_html(r#"
            <p>Error config websocket</p>
        "#);
    }

    Ok(())
}
