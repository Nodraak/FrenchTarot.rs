use uuid::Uuid;
use web_sys;
use wasm_bindgen::prelude::*;

mod js_api;
mod utils;
mod view;
mod websocket;


fn wrapped_main(document: &web_sys::Document, game_uuid: Uuid) -> utils::Result<()> {
    view::init(&document, game_uuid)?;

    Ok(())
}


#[wasm_bindgen]
pub fn main(game_uuid_: String, username: String) -> utils::Result<()> {
    let game_uuid = Uuid::parse_str(&game_uuid_).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    document.query_selector("#main > p").unwrap().unwrap().set_inner_html("Starting game... (2/3)");

    let r = wrapped_main(&document, game_uuid);

    if let Err(e) = r {
        let main = document.get_element_by_id("main").unwrap();
        main.set_inner_html(r#"
            <p>Error 1</p>
        "#);
    }

    let ws_host = "127.0.0.1:8001";
    let ws_path = format!("/game/play/{}", game_uuid.to_string());

    let r = websocket::main(format!("ws://{}{}", ws_host, ws_path), username);
    if let Err(e) = r {
        let main = document.get_element_by_id("main").unwrap();
        main.set_inner_html(r#"
            <p>Error 2</p>
        "#);
    }

    Ok(())
}
