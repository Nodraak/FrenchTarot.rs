use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn on_open(ws: &WebSocket, v: JsValue) {
    console_log!("ws opened {:?}", v);
    match ws.send_with_str("ping") {
        Ok(ret) => console_log!("message successfully sent {:?}", ret),
        Err(err) => console_log!("error sending message: {:?}", err),
    }
}

fn on_error(error: ErrorEvent) {
    console_log!("error event: {:?}", error);
}

fn on_message(msg: MessageEvent) {
    let data = msg
        .data()
        .as_string()
        .expect("Can't convert received data to a string");
    console_log!("message event, received data: {:?}", data);
}


pub fn main(addr: &str) -> Result<(), JsValue> {
    let ws = WebSocket::new(addr)?;

    let on_open_ws = ws.clone();
    let c = Closure::wrap(Box::new(move |v| { on_open(&on_open_ws, v); }) as Box<dyn FnMut(JsValue)>);
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
