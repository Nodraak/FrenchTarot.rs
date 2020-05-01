#![feature(proc_macro_hygiene, decl_macro)]
#![feature(vec_remove_item)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::thread;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod conf;
mod db;
mod http_routes;
mod websockets;


fn main() {
    thread::spawn(|| {websockets::server::main()});

    rocket::ignite()
        // routes
        .mount("/", http_routes::index::get_routes())
        .mount("/api", http_routes::api::get_routes())
        .mount("/user", http_routes::user::get_routes())
        .mount("/game", http_routes::game::get_routes())
        // db, templates and static files
        .attach(db::utils::DbConn::fairing())
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("./static"))
        // launch!
        .launch();
}
