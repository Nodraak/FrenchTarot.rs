use rocket_contrib::databases;

#[database("sqlite_logs")]
pub struct DbConn(databases::diesel::SqliteConnection);
