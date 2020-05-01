use std::collections::HashMap;

use rocket::http::RawStr;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;
use uuid::Uuid;

use tarot_lib::game::game::Game as GameObj;

use crate::db::models;
use crate::db::utils::DbConn;
use crate::http_routes::utils::User;


#[derive(FromForm)]
pub struct GameCreateForm {
    max_players_count: i32,
}

impl FromFormValue<'_> for GameCreateForm {
    type Error = &'static str;

    fn from_form_value(v: &RawStr) -> Result<Self, Self::Error> {
        let max_players_count = match i32::from_form_value(v) {
            Ok(v) => v,
            Err(_) => return Err("value is not a number."),
        };

        if (2 <= max_players_count) && (max_players_count <= 5) {
            Ok(GameCreateForm { max_players_count: max_players_count })
        } else {
            Err("Players must be 2 <= and <= 5")
        }
    }
}


pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        index,
        create_get,
        create_post,
        play,
    ]
}


#[get("/")]
pub fn index(user: User, conn: DbConn) -> Template {

    #[derive(Serialize)]
    struct Context {
        games: Vec<GameObj>,
        current_players_count: usize,
    }

    let games = models::game::list(&conn);
    let c = games.len();

    Template::render("game/index", Context {
        games: games,
        current_players_count: c,
    })
}

#[get("/create")]
pub fn create_get(user: User) -> Template {
    let context = HashMap::<&str, &str>::new();
    Template::render("game/create", &context)
}

#[post("/create", data = "<game>")]
pub fn create_post(game: Form<GameCreateForm>, user: User, conn: DbConn) -> Result<Redirect, String> {
    let game_uuid = Uuid::new_v4();

    models::game::create(&conn, GameObj {
        uuid: game_uuid,
        max_players_count: game.max_players_count,
        creator_uuid: user.uuid,
        players: vec![],
    });

    Ok(Redirect::to(format!("/game/play/{}", game_uuid.to_string())))
}

#[get("/play/<uuid>")]
pub fn play(uuid: String, user: User) -> Template {
    let mut context = HashMap::<&str, String>::new();
    context.insert("game_uuid", uuid);
    context.insert("user_uuid", user.uuid.to_string());
    Template::render("game/play", &context)
}
