
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;
//use std::sync::atomic::{AtomicUsize, Ordering};

//use rocket::request::{self, Form, FromRequest, Request};
//use rocket::Request;
//use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
//use serde::Serialize;

//use tarot_lib;


#[get("/")]
fn index() -> Template {
    let context = HashMap::<&str, &str>::new();
    Template::render("index", &context)
}

mod user {
    use std::collections::HashMap;
    use rocket_contrib::templates::Template;

    #[get("/")]
    pub fn get() -> Template {
        let context = HashMap::<&str, &str>::new();
        Template::render("user/index", &context)
    }

    #[post("/")]
    pub fn post() -> Template {
        let context = HashMap::<&str, &str>::new();
        Template::render("user/index", &context)
    }
}

mod game {
    use std::collections::HashMap;
    use rocket_contrib::templates::Template;

    #[get("/")]
    pub fn index() -> Template {
        let context = HashMap::<&str, &str>::new();
        Template::render("game/index", &context)
    }

    #[get("/create")]
    pub fn create_get() -> Template {
        let context = HashMap::<&str, &str>::new();
        Template::render("game/create", &context)
    }

    #[post("/create")]
    pub fn create_post() -> Template {
        let context = HashMap::<&str, &str>::new();
        Template::render("game/create", &context)
    }

    #[get("/play/<id>")]
    pub fn play(id: String) -> Template {
        let context = HashMap::<&str, &str>::new();
        Template::render("game/play", &context)
    }
}


fn main() {
    tarot_lib::main();
    rocket::ignite()
        // routes
        .mount("/", routes![index])
        .mount("/user", routes![user::get, user::post])
        .mount("/game", routes![game::index, game::create_get, game::create_post, game::play])
        // templates and static
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("./static"))
        // launch!
        .launch();
}
