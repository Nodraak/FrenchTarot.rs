
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;

use rocket::http::{Cookie, RawStr};
use rocket::request::{self, FromRequest, FromFormValue, Request};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

//use tarot_lib;


#[derive(FromForm)]
pub struct User {
    username: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = &'static str;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        let cookie = request.cookies().get_private("username");
        match cookie {
            Some(c) => {
                rocket::Outcome::Success(User {username: c.value().to_string() })
            }
            None => {
                let username = "Guest42";  // TODO: generate name

                // TODO store in db
                request.cookies().add_private(Cookie::new("username", username));

                rocket::Outcome::Success(User {username: username.to_string() })
            }
        }
    }
}


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
fn index(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.username);
    Template::render("index", &context)
}

mod user {
    use std::collections::HashMap;
    use rocket::http::{Cookie, Cookies};
    use rocket::request::{Form};
    use rocket::response::Redirect;
    use rocket_contrib::templates::Template;
    use crate::User;

    #[get("/")]
    pub fn get(user: User) -> Template {
        let mut context = HashMap::new();
        context.insert("username", user.username);
        Template::render("user/index", &context)
    }

    #[post("/", data = "<user>")]
    pub fn post(user: Form<User>, mut cookies: Cookies) -> Result<Redirect, String> {
        // TODO check in db it does not already exists

        // TODO store in db
        cookies.add_private(Cookie::new("username", user.username.clone()));

        Ok(Redirect::to("/"))
    }
}

mod game {
    use std::collections::HashMap;
    use rocket::request::{Form};
    use rocket::response::Redirect;
    use rocket_contrib::templates::Template;
    use crate::{User, GameCreate};

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

        Ok(Redirect::to(format!("/game/play/{}", game_id)))
    }

    #[get("/play/<id>")]
    pub fn play(id: String, user: User) -> Template {
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
