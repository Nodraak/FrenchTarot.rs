use diesel::sqlite::SqliteConnection;

use super::schema::games;

use crate::diesel::RunQueryDsl;

//use tarot_lib::player::Player;

#[derive(Queryable)]
pub struct Game {
//pub struct Game<'a> {
    pub pk: i32,
    pub players: i32,
//    pub creator: &'a Player,
//    pub players: [&'a Player; 5],
}

#[derive(Insertable)]
#[table_name="games"]
pub struct GameNew {
    pub players: i32,
}

pub fn game_create(conn: &SqliteConnection, players: i32) {
    let g = GameNew {
        players: players,
    };

    diesel::insert_into(games::table)
        .values(&g)
        .execute(conn)
        .expect("Error saving new post");
}
