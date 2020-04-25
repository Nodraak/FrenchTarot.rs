use uuid::Uuid;
use wasm_bindgen::prelude::*;

use tarot_lib;

mod js_api;
use js_api::alert;

fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn main(game_id_: String) -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");


    let game_id = Uuid::parse_str(&game_id_).unwrap();

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    greet(&game_id_);

    tarot_lib::main();

    Ok(())
}
