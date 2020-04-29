use tarot_lib::game::events::Event;

use crate::js_api::log;


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


pub fn events_append_str(msg: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let events = document.get_element_by_id("events").unwrap();
    events.set_inner_html(&(events.inner_html() + msg));
}


pub fn update(msg: &Event) {
    match msg {
        Event::WsConnect(data) => {
            console_log!("on_message(): {:?}", data.username);
            events_append_str(&format!(
                r#"
                    <p>
                        {} connected!
                    </p>
                "#,
                data.username,
            ));
        },
        Event::WsDisconnect(data) => {
            console_log!("on_message(): {:?}", data.username);
            events_append_str(&format!(
                r#"
                    <p>
                        {} disconnected.
                    </p>
                "#,
                data.username,
            ));
        },
        _ => {
            console_log!("on_message(): Not handled (yet) {:?}", msg);
        },
    }
}
