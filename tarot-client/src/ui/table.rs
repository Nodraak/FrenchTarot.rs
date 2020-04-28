use uuid::Uuid;
use web_sys;

use crate::utils;

pub fn init(document: &web_sys::Document, game_uuid: Uuid) -> utils::Result<()> {
    let main = document.get_element_by_id("main").unwrap();
    main.set_inner_html(r#"
        <div id="table">
        </div>
        <div id="info">
            <div id="game">
            </div>
            <div id="events">
            </div>
        </div>
    "#);

    Ok(())
}
