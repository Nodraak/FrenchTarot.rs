use tarot_lib::game::events::Event;

use crate::js_api::log;


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


pub fn events_append_str(msg: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let events = document.get_element_by_id("events").unwrap();
    let e = format!("<p>{}</p>", msg);
    events.set_inner_html(&(events.inner_html() + &e));
}


pub fn update(msg: &Event) {
    match msg {
        Event::WsConnect(data) => {
            events_append_str(&format!("{} connected!", data.username));
        },
        Event::WsDisconnect(data) => {
            events_append_str(&format!("{} disconnected.", data.username));
        },
        Event::Game(data) => {
            console_log!("game: {:?}", data);
            // TODO
        },
        Event::GameJoin(data) => {
            events_append_str(&format!("{} joined the game.", data.username));
        },
        _ => {
            console_log!("on_message(): Not handled (yet) {:?}", msg);
        },
    }
}
