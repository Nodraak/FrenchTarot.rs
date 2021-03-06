use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket_contrib::json::Json;
use uuid::Uuid;

use tarot_lib::game::game::Game as GameObj;
use tarot_lib::player::Player as PlayerObj;

use crate::db::models;
use crate::db::utils::DbConn;


pub struct Internal;

impl<'a, 'r> FromRequest<'a, 'r> for Internal {
    type Error = &'static str;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.client_ip() {
            None => {
                rocket::Outcome::Failure((Status::BadRequest, "no ip"))
            }
            Some(ip) => {
                if ip.is_loopback() {
                    rocket::Outcome::Success(Internal {})
                }
                else {
                    rocket::Outcome::Failure((Status::Forbidden, "not loopback"))
                }
            }
        }
    }
}


pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        game_get,
        game_update,
        player_get,
    ]
}


#[get("/game/get/<uuid>")]
pub fn game_get(uuid: String, conn: DbConn, _i: Internal) -> Json<GameObj> {
    let g = models::game::get(&conn, Uuid::parse_str(&uuid).unwrap());
    Json(g)
}


#[post("/game/update", data = "<game>")]
pub fn game_update(game: Json<GameObj>, conn: DbConn, _i: Internal) {
    let g: GameObj = game.into_inner();
    models::game::update(&conn, g.uuid, g);
}


#[get("/player/get/<uuid>")]
pub fn player_get(uuid: String, conn: DbConn, _i: Internal) -> Json<PlayerObj> {
    let u = models::user::get(&conn, Uuid::parse_str(&uuid).unwrap());
    Json(PlayerObj {
        uuid: u.uuid,
        username: u.username,
    })
}
