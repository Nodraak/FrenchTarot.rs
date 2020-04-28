use serde_json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

use tarot_lib::game::{events, state_machine};


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn on_open(ws: &WebSocket, username: String, v: JsValue) {
    console_log!("on_open(): {:?}", v);

    let event = state_machine::Event::WsConnect(events::data::WsConnectData {
        username: username,
    });
    let ret = ws.send_with_str(&serde_json::to_string(&event).unwrap());

    if let Err(err) = ret {
        console_log!("error sending message: {:?}", err);
    }
}

fn on_error(error: ErrorEvent) {
    console_log!("on_error(): {:?}", error);
}

fn on_message(msg: MessageEvent) {
    let data = msg
        .data()
        .as_string()
        .expect("Can't convert received data to a string");
    let deserialized: state_machine::Event = serde_json::from_str(&data).unwrap();

    console_log!("on_message(): {:?}", deserialized);

    match deserialized {
        state_machine::Event::WsConnect(data) => {
            console_log!("on_message(): {:?}", data.username);

            let document = web_sys::window().unwrap().document().unwrap();
            let info = document.get_element_by_id("info").unwrap();
            info.set_inner_html(&format!(r#"
                <p>
                    {} connected!
                </p>
            "#, data.username));
        }
        _ => {
            console_log!("on_message(): Not handled (yet) {:?}", deserialized);
        }
    }
}


pub fn main(addr: String, username: String) -> Result<(), JsValue> {
    let ws = WebSocket::new(&addr)?;

    let on_open_ws = ws.clone();
    let c = Closure::wrap(Box::new(move |v| { on_open(&on_open_ws, username.clone(), v); }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(c.as_ref().unchecked_ref()));
    c.forget();

    // TODO on_close

    let c = Closure::wrap(Box::new(move |e| { on_error(e); }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(c.as_ref().unchecked_ref()));
    c.forget();

    let c = Closure::wrap(Box::new(move |e| { on_message(e); }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(c.as_ref().unchecked_ref()));
    c.forget();

    Ok(())
}