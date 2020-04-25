use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use uuid::Uuid;

use tarot_lib::game::Game as GameObj;
use tarot_lib::player::Player as PlayerObj;

use crate::db::models::Game as GameData;
use crate::db::models::GamePlayers as GamePlayersData;
use crate::db::models::User as UserData;
use crate::db::schema::{users, games, game_players};


pub fn create(conn: &SqliteConnection, game_obj: GameObj) {
    let creator_uuid: Option<String> = match game_obj.creator {
        None => { None },
        Some(c) => { Some(c.uuid.to_string()) },
    };

    // create game

    diesel::insert_into(games::table)
        .values(GameData {
            uuid: game_obj.uuid.to_string(),
            max_players_count: game_obj.max_players_count,
            creator_uuid: creator_uuid,
        })
        .execute(conn)
        .expect("Error saving new game");

    // add game players

    for p in game_obj.players {
        diesel::insert_into(game_players::table)
            .values(GamePlayersData {
                uuid: Uuid::new_v4().to_string(),
                user_uuid: Some(p.uuid.to_string()),
                game_uuid: Some(game_obj.uuid.to_string()),
            })
            .execute(conn)
            .expect("Error saving new game player");
    }
}

pub fn list(conn: &SqliteConnection) -> Vec<GameObj> {
    use crate::db::schema::games::dsl::games;

    // TODO paginate
    let page_size = 100;
    let page_id = 0;

    let results = games
        .limit(page_size)
        .offset(page_size*page_id)
        .load::<GameData>(conn)
        .expect("Error loading games");

    let mut ret = Vec::<GameObj>::new();
    for g in results {
        ret.push(GameObj {
            uuid: Uuid::parse_str(&g.uuid).unwrap(),
            max_players_count: g.max_players_count,
            creator: None,  // TODO get
            players: vec![],  // TODO get
        });
    }

    ret
}
