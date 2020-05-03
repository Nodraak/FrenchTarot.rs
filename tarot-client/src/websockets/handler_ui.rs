use tarot_lib::game::events::Event;

use crate::js_api::log;
use crate::ui::info;


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


pub fn update(msg: &Event) {
    match msg {
        Event::WsConnect(data) => {
            info::events_append_str(&format!("{} connected!", data.username));
        },
        Event::WsDisconnect(data) => {
            info::events_append_str(&format!("{} disconnected.", data.username));
        },
        Event::Game(data) => {
            info::game_update(data);
        },
        Event::GameJoin(data) => {
            info::events_append_str(&format!("{} joined the game.", data.username));
        },
        _ => {
            console_log!("on_message(): Not handled (yet) {:?}", msg);
        },
    }
}
