use serde::Serialize;

use super::schema::{users, games, game_players};

type Uuid = String;  // Uuid not supported for diesel sqlite

#[derive(Insertable, Queryable)]
#[derive(Serialize)]
#[table_name="users"]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
}

#[derive(Insertable, Queryable)]
#[derive(Serialize)]
#[table_name="games"]
pub struct Game {
    pub uuid: Uuid,
    pub max_players_count: i32,
    pub creator_uuid: Option<Uuid>,  // User.Uuid
}

#[derive(Insertable, Queryable)]
#[derive(Serialize)]
#[table_name="game_players"]
pub struct GamePlayers {
    pub uuid: Uuid,
    pub user_uuid: Option<Uuid>,  // User.uuid
    pub game_uuid: Option<Uuid>,  // Game.uuid
}
