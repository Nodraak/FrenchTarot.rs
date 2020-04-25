#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use diesel::prelude::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod db;
use db::models::*;
use db::utils;

mod routes;


fn main() {
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
