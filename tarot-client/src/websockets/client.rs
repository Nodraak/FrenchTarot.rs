use std::sync::{Arc, Mutex};

use serde_json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

use tarot_lib::game::events::Event;
use tarot_lib::game::game::Game;

use crate::js_api::log;
use crate::websockets::handler_ui;


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


struct GameData {
    game: Option<Game>,
}


fn on_open(_v: JsValue) {
}

fn on_close(_error: ErrorEvent) {
    handler_ui::events_append_str("Disconnected from server");
}

fn on_error(_error: ErrorEvent) {
    handler_ui::events_append_str("Connection error. Try refreshing the page. Guru meditation: on_error()");
}

fn on_message(game_data_: Arc<Mutex<GameData>>, msg: MessageEvent) {
    let mut game_data = game_data_.lock().unwrap();

    let payload = match msg.data().as_string() {
        None => {
            handler_ui::events_append_str("Connection error. Try refreshing the page. Guru meditation: on_message() read payload");
            return;
        },
        Some(payload) => {
            payload
        },
    };

    let event: Event = match serde_json::from_str(&payload) {
        Err(_) => {
            handler_ui::events_append_str("Connection error. Try refreshing the page. Guru meditation: on_message() deserialization");
            return;
        },
        Ok(event) => {
            event
        }
    };

    console_log!("on_message(): {:?}", event);

    // set or update game
    let ret = match &event {
        Event::Game(data) => {
            game_data.game = Some(data.game.clone());
            Ok(())
        },
        _ => {
            game_data.game.as_mut().expect("game is None").update(&event)
        },
    };
    if let Err(_) = ret {
        handler_ui::events_append_str("Connection error. Try refreshing the page. Guru meditation: on_message() update game");
        return;
    }

    // update ui
    handler_ui::update(&event);
}


pub fn main(addr: String) -> Result<(), JsValue> {
    let ws = WebSocket::new(&addr)?;

    let game_data = Arc::new(Mutex::new(GameData {
        game: None,
    }));

    let c = Closure::wrap(Box::new(move |v| { on_open(v); }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(c.as_ref().unchecked_ref()));
    c.forget();

    let c = Closure::wrap(Box::new(move |e| { on_close(e); }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onclose(Some(c.as_ref().unchecked_ref()));
    c.forget();

    let c = Closure::wrap(Box::new(move |e| { on_error(e); }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(c.as_ref().unchecked_ref()));
    c.forget();

    let c = Closure::wrap(Box::new(move |e| { on_message(Arc::clone(&game_data), e); }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(c.as_ref().unchecked_ref()));
    c.forget();

    Ok(())
}
