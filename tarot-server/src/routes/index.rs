use std::collections::HashMap;

use rocket_contrib::templates::Template;

use crate::routes::utils::User;


#[get("/")]
pub fn index(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.username);
    Template::render("index", &context)
}
