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


fn main() {

    tarot_lib::main();

    let connection = utils::connect();
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
