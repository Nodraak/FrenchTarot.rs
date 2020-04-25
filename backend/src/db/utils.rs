use std::env;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use rocket_contrib::databases;

#[database("sqlite_logs")]
pub struct DbConn(databases::diesel::SqliteConnection);


pub fn connect() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
