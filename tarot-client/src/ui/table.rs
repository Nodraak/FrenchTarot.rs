use uuid::Uuid;
use web_sys;

use crate::utils;

pub fn init(document: &web_sys::Document, game_uuid: Uuid) -> utils::Result<()> {
    let main = document.get_element_by_id("main").unwrap();
    main.set_inner_html(&format!(
        r#"
            <div id="table">
            </div>
            <div id="info">
                <div id="game">
                    <ul>
                        <li>Id: <a href="/game/play/{}">{}</a></li>
                    </ul>
                </div>
                <div id="events">
                    <p>Connecting...</p>
                </div>
            </div>
        "#,
        game_uuid.to_string(), game_uuid.to_string(),
    ));

    Ok(())
}
