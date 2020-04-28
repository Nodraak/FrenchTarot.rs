use std::collections::HashMap;
use uuid::Uuid;

use rocket::http::RawStr;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use tarot_lib::game::game::Game as GameObj;

use crate::db::accessors;
use crate::db::utils::DbConn;
use crate::routes::user::User;


#[derive(FromForm)]
pub struct GameCreate {
    max_players_count: i32,
}

impl FromFormValue<'_> for GameCreate {
    type Error = &'static str;

    fn from_form_value(v: &RawStr) -> Result<Self, Self::Error> {
        let max_players_count = match i32::from_form_value(v) {
            Ok(v) => v,
            Err(_) => return Err("value is not a number."),
        };

        if (2 <= max_players_count) && (max_players_count <= 5) {
            Ok(GameCreate { max_players_count: max_players_count })
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
pub fn create_post(game: Form<GameCreate>, user: User, conn: DbConn) -> Result<Redirect, String> {
    let game_uuid = Uuid::new_v4();

    // TODO update db with game.players

    accessors::game::create(&conn, GameObj {
        uuid: game_uuid,
        max_players_count: game.max_players_count,
        creator: None,
        players: vec![],
    });

    // TODO create the game state machine

    Ok(Redirect::to(format!("/game/play/{}", game_uuid.to_string())))
}

#[get("/play/<id>")]
pub fn play(id: String, user: User) -> Template {
    // TODO check join as player or watcher

    let mut context = HashMap::<&str, String>::new();
    context.insert("game_id", id);
    context.insert("username", user.username.clone());
    Template::render("game/play", &context)
}
