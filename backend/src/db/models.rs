use super::schema::{users, games, game_players};


#[derive(Queryable)]
pub struct User {
    pub pk: i32,
    pub username: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct UserNew {
    pub username: String,
}

#[derive(Queryable)]
pub struct Game {
    pub pk: i32,
    pub max_players_count: i32,
    pub creator_pk: Option<i32>,  // User.pk
}

#[derive(Insertable)]
#[table_name="games"]
pub struct GameNew {
    pub max_players_count: i32,
    pub creator_pk: Option<i32>,  // User.pk
}

#[derive(Queryable)]
pub struct GamePlayers {
    pub pk: i32,
    pub user_pk: Option<i32>,  // User.pk
    pub game_pk: Option<i32>,  // Game.pk
}

#[derive(Insertable)]
#[table_name="game_players"]
pub struct GamePlayersNew {
    pub user_pk: Option<i32>,  // User.pk
    pub game_pk: Option<i32>,  // Game.pk
}
