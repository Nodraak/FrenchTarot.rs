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
pub fn main(game_uuid_: String) -> utils::Result<()> {
    let game_uuid = Uuid::parse_str(&game_uuid_).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    document.query_selector("#main > p").unwrap().unwrap().set_inner_html("Starting game... (2/3)");

    let r = wrapped_main(&document, game_uuid);

    websocket::main("ws://127.0.0.1:8001");

    match r {
        Ok(v) => {},
        Err(e) => {
            let main = document.get_element_by_id("main").unwrap();
            main.set_inner_html(r#"
                <p>Error</p>
            "#);
            js_api::alert("Error");
        },
    };

    Ok(())
}
