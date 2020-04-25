#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use ::uuid::Uuid;

use diesel::prelude::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

//use tarot_lib;

mod db;
use db::accessors;
use db::models::*;
use db::schema::games::dsl::*;
use db::utils;

mod routes;

use tarot_lib::game::Game as GameObj;
use tarot_lib::player::Player as PlayerObj;


fn main() {

    tarot_lib::main();

    let connection = utils::connect();

    accessors::game::create(&connection, GameObj {
        uuid: Uuid::new_v4(),
        max_players_count: 5,
        creator: None,
        players: vec![],
    });

    let results = games
        .limit(20)
        .load::<Game>(&connection)
        .expect("Error loading games");

    println!("Displaying {} games", results.len());
    for post in results {
        println!("* {:?}", post.uuid);
    }


    rocket::ignite()
        // routes
        .mount("/", routes![routes::index::index])
        .mount("/user", routes![routes::user::get, routes::user::post])
        .mount("/game", routes![routes::game::index, routes::game::create_get, routes::game::create_post, routes::game::play])
        // db, templates and static files
        .attach(utils::DbConn::fairing())
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("./static"))
        // launch!
        .launch();
}
