use std::collections::HashMap;

use rocket::http::{Cookie, Cookies};
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::Redirect;
use rocket_contrib::templates::Template;


#[derive(FromForm)]
pub struct User {
    pub username: String,
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
