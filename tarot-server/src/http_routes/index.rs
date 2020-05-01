use std::collections::HashMap;

use rocket_contrib::templates::Template;

use crate::http_routes::utils::User;


pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        index,
    ]
}


#[get("/")]
fn index(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.username);
    Template::render("index", &context)
}
