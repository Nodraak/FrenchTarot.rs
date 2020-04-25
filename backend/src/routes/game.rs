use std::collections::HashMap;

use rocket::http::RawStr;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::routes::user::User;


#[derive(FromForm)]
pub struct GameCreate {
    players: i8,
}

impl FromFormValue<'_> for GameCreate {
    type Error = &'static str;

    fn from_form_value(v: &RawStr) -> Result<Self, Self::Error> {
        let players = match i8::from_form_value(v) {
            Ok(v) => v,
            Err(_) => return Err("value is not a number."),
        };

        if (2 <= players) && (players <= 5) {
            Ok(GameCreate { players })
        } else {
            Err("Players must be 2 <= and <= 5")
        }
    }
}


#[get("/")]
pub fn index(user: User) -> Template {
    let context = HashMap::<&str, &str>::new();
    // TODO read games from db
    Template::render("game/index", &context)
}

#[get("/create")]
pub fn create_get(user: User) -> Template {
    let context = HashMap::<&str, &str>::new();
    Template::render("game/create", &context)
}

#[post("/create", data = "<game>")]
pub fn create_post(game: Form<GameCreate>, user: User) -> Result<Redirect, String> {
    let context = HashMap::<&str, &str>::new();

    let game_id = "gameid42";  // TODO generate game id

    // TODO update db with game.players and game_id
    // TODO create the game state machine

    Ok(Redirect::to(format!("/game/play/{}", game_id)))
}

#[get("/play/<id>")]
pub fn play(id: String, user: User) -> Template {
    let context = HashMap::<&str, &str>::new();
    Template::render("game/play", &context)
}

// #[post("/api", data = "<data>")]
// pub fn api() {
//
// }
