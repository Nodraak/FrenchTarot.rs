use rocket_contrib::templates::Template;

use crate::db::accessors::game;
use crate::db::utils::DbConn;
use crate::routes::user::User;

use tarot_lib::game::game::Game;


#[get("/")]
pub fn index(user: User, conn: DbConn) -> Template {

    // context boilerplate

    use serde::ser::{Serialize, Serializer, SerializeStruct};

    struct Context {
        username: String,
        games: Vec<Game>,
    }

    impl Serialize for Context {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // 3 is the number of fields in the struct.
            let mut state = serializer.serialize_struct("Context", 2)?;
            state.serialize_field("username", &self.username)?;
            state.serialize_field("games", &self.games)?;
            state.end()
        }
    }

    // actual function

    let context = Context {
        username: user.username,
        games: game::list(&conn),
    };

    Template::render("index", &context)
}
