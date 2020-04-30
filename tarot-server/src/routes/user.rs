use std::collections::HashMap;

use rocket::http::Cookie;
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use uuid::Uuid;

use crate::db::accessors::users;
use crate::db::utils::DbConn;
use crate::routes::utils::User;


#[derive(FromForm)]
pub struct UserCreateForm {
    pub username: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = &'static str;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let conn = DbConn::from_request(request).unwrap();

        let cookie = request.cookies().get_private("uuid");
        let u: User = match cookie {
            // existing user
            Some(c) => {
                users::get(&conn, Uuid::parse_str(&c.value().to_string()).unwrap())
            }
            // new user
            None => {
                let username = "Guest42".to_string();

                let u = User {
                    uuid: Uuid::new_v4(),
                    username: username,
                };

                users::create(&conn, &u);
                request.cookies().add_private(Cookie::new("uuid", u.uuid.to_string()));

                u
            }
        };

        rocket::Outcome::Success(u)
    }
}


#[get("/")]
pub fn get(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.username);
    Template::render("user/index", &context)
}

#[post("/", data = "<user_data>")]
pub fn post(user_session: User, user_data: Form<UserCreateForm>, conn: DbConn) -> Result<Redirect, String> {
    users::update(&conn, user_session.uuid, User {
        uuid: user_session.uuid,
        username: user_data.username.clone(),
    });

    Ok(Redirect::to("/"))
}
