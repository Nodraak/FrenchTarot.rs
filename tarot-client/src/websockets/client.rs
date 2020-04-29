use std::sync::{Arc, Mutex};

use serde_json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

use tarot_lib::game::events::Event;
use tarot_lib::game::events_data;
use tarot_lib::game::game::Game;

use crate::websockets::handler_ui;
use crate::js_api::log;


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


struct GameData {
    username: String,
    game: Option<Game>,
    socket: WebSocket,
}


fn on_open(ws: &WebSocket, username: String, v: JsValue) {
    console_log!("on_open(): {:?}", v);

    let event = Event::WsConnect(events_data::WsConnectData {
        username: username,
    });
    let ret = ws.send_with_str(&serde_json::to_string(&event).unwrap());

    if let Err(err) = ret {
        console_log!("error sending message: {:?}", err);
    }
}

fn on_close(error: ErrorEvent) {
    console_log!("on_close(): {:?}", error);

    handler_ui::events_append_str("Disconnect from server");
}

fn on_error(error: ErrorEvent) {
    console_log!("on_error(): {:?}", error);

    handler_ui::events_append_str("Connection error. Try refreshing the page.");
}

fn on_message(game_data_: Arc<Mutex<GameData>>, msg: MessageEvent) {
    let mut game_data = game_data_.lock().unwrap();

    let data = msg
        .data()
        .as_string()
        .expect("Can't convert received data to a string");
    let deserialized: Event = serde_json::from_str(&data).unwrap();

    console_log!("on_message(): {:?}", deserialized);

    // set or update game
    let ret = match &deserialized {
        Event::CreateGame(data) | Event::GameJoin(data) => {
            game_data.game = Some(data.game.clone());
            Ok(())
        },
        _ => {
            game_data.game.as_mut().unwrap().update(&deserialized)
        },
    };

    // update ui
    match ret {
        Ok(_) => { handler_ui::update(&deserialized); },
        Err(val) => { panic!(val); }, // TODO proper error handling
    }
}


pub fn main(addr: String, username: String) -> Result<(), JsValue> {
    let ws = WebSocket::new(&addr)?;

    let game_data = Arc::new(Mutex::new(GameData {
        username: username.clone(),
        game: None,
        socket: ws.clone(),
    }));

    let ws_onopen = ws.clone();
    let c = Closure::wrap(Box::new(move |v| { on_open(&ws_onopen, username.clone(), v); }) as Box<dyn FnMut(JsValue)>);
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
