use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;

use uuid::Uuid;

use super::models::Game as GameData;
use super::models::GamePlayers as GamePlayersData;
use super::models::User as UserData;
use super::schema::{users, games, game_players};

use tarot_lib::game::Game as GameObj;
use tarot_lib::player::Player as PlayerObj;


pub fn game_create(conn: &SqliteConnection, game_obj: GameObj) {
    let game_uuid = Uuid::new_v4();

    let creator_uuid: Option<String> = match game_obj.creator {
        None => { None },
        Some(c) => { Some(c.uuid.to_string()) },
    };

    // create game

    diesel::insert_into(games::table)
        .values(GameData {
            uuid: game_uuid.to_string(),
            max_players_count: game_obj.max_players_count,
            creator_uuid: creator_uuid,
        })
        .execute(conn)
        .expect("Error saving new post");

    // add game players

    for p in game_obj.players {
        diesel::insert_into(game_players::table)
            .values(GamePlayersData {
                uuid: Uuid::new_v4().to_string(),
                user_uuid: Some(p.uuid.to_string()),
                game_uuid: Some(game_uuid.to_string()),
            })
            .execute(conn)
            .expect("Error saving new post");
    }
}
