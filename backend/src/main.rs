#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

use diesel::prelude::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

//use tarot_lib;

mod db;
use db::models::*;
use db::schema::games::dsl::*;
use db::utils;
mod routes;
use routes::{index, user, game};

use tarot_lib::game::Game as GameObj;
use tarot_lib::player::Player as PlayerObj;

use db::accessors::game_create;

fn main() {

    tarot_lib::main();

    let connection = utils::connect();

    game_create(&connection, GameObj {
        pk: 42,
        max_players_count: 43,
        creator: None,
        players: None,

    });
    game_create(&connection, GameObj {
        pk: 42,
        max_players_count: 43,
        creator: None,
        players: None,

    });
    game_create(&connection, GameObj {
        pk: 42,
        max_players_count: 43,
        creator: None,
        players: None,

    });

    let results = games
        .limit(5)
        .load::<Game>(&connection)
        .expect("Error loading games");

    println!("Displaying {} games", results.len());
    for post in results {
        println!("{}", post.pk);
    }


    rocket::ignite()
        // routes
        .mount("/", routes![index::index])
        .mount("/user", routes![user::get, user::post])
        .mount("/game", routes![game::index, game::create_get, game::create_post, game::play])
        // templates and static
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("./static"))
        // launch!
        .launch();
}
