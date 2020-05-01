use rocket_contrib::databases;


// Uuid not supported for diesel sqlite
pub type Uuid = String;


#[database("sqlite_logs")]
pub struct DbConn(databases::diesel::SqliteConnection);
