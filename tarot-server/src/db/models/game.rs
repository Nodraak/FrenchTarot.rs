use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use serde::Serialize;
use uuid;

use tarot_lib::game::game::Game as GameObj;
use tarot_lib::player::Player as PlayerObj;

use crate::db::models::game::Game as GameData;
use crate::db::models::game::GamePlayers as GamePlayersData;
use crate::db::models::user::User as UserData;
use crate::db::schema::{games, game_players, users};
use crate::db::utils;


#[derive(Insertable, Queryable)]
#[derive(Serialize)]
#[table_name="games"]
pub struct Game {
    pub uuid: utils::Uuid,
    pub max_players_count: i32,
    pub creator_uuid: Option<utils::Uuid>,  // User.Uuid
}

#[derive(Insertable, Queryable)]
#[derive(Serialize)]
#[table_name="game_players"]
pub struct GamePlayers {
    pub uuid: utils::Uuid,
    pub user_uuid: Option<utils::Uuid>,  // User.uuid
    pub game_uuid: Option<utils::Uuid>,  // Game.uuid
}


pub fn create(conn: &SqliteConnection, game_obj: GameObj) {
    // create game

    diesel::insert_into(games::table)
        .values(GameData {
            uuid: game_obj.uuid.to_string(),
            max_players_count: game_obj.max_players_count,
            creator_uuid: Some(game_obj.creator_uuid.to_string()),
        })
        .execute(conn)
        .expect("Error saving new game");

    // add game players

    for p in game_obj.players {
        diesel::insert_into(game_players::table)
            .values(GamePlayersData {
                uuid: uuid::Uuid::new_v4().to_string(),
                user_uuid: Some(p.uuid.to_string()),
                game_uuid: Some(game_obj.uuid.to_string()),
            })
            .execute(conn)
            .expect("Error saving new game player");
    }
}

pub fn get(conn: &SqliteConnection, uuid: uuid::Uuid) -> GameObj {

    // get game_players

    let results = game_players::dsl::game_players
        .filter(game_players::dsl::game_uuid.eq(uuid.to_string()))
        .load::<GamePlayersData>(conn)
        .expect("Error loading GamePlayers");

    let mut players = Vec::new();

    for gp in results {
        let p = users::dsl::users
            .filter(users::dsl::uuid.eq(&gp.uuid))
            .load::<UserData>(conn)
            .expect("Error loading User");

        players.push(PlayerObj {
            uuid: uuid::Uuid::parse_str(&gp.uuid).unwrap(),
            username: p[0].username.clone(),
        });
    }

    // get game

    let results = games::dsl::games
        .filter(games::dsl::uuid.eq(uuid.to_string()))
        .load::<GameData>(conn)
        .expect("Error loading games");

    if results.len() != 1 {
        panic!("oh god - game get");
    }

    let g = GameObj {
        uuid: uuid::Uuid::parse_str(&results[0].uuid).unwrap(),
        max_players_count: results[0].max_players_count,
        creator_uuid: uuid::Uuid::parse_str(&results[0].creator_uuid.as_ref().unwrap()).unwrap(),
        players: players,
    };

    g
}

pub fn update(conn: &SqliteConnection, uuid: uuid::Uuid, game_obj: GameObj) {

    diesel::update(games::table).set(
        GameData {
            uuid: game_obj.uuid.to_string(),
            max_players_count: game_obj.max_players_count,
            creator_uuid: Some(game_obj.creator_uuid.to_string()),
        }
    );

    // update players

    let results = game_players::dsl::game_players
        .filter(game_players::dsl::game_uuid.eq(game_obj.uuid.to_string()))
        .load::<GamePlayersData>(conn)
        .expect("Error loading GamePlayers");

    /*
    TODO delete old gp

    for gp in results {
        if game_obj.players.contains(PlayerObj {
            uuid: uuid::Uuid::parse_str(&gp.uuid).unwrap(),
            username: gp.username,
        }) == false {
            diesel::delete(
                    game_players::dsl::game_players.filter(game_players::dsl::uuid.eq(gp.uuid))
                )
                .execute(&conn)
                .expect("Error deleting gp");
        }
    }
    */

    for p in game_obj.players {
        if (
            game_players::dsl::game_players
                .filter(game_players::dsl::game_uuid.eq(game_obj.uuid.to_string()))
                .filter(game_players::dsl::user_uuid.eq(p.uuid.to_string()))
                .count()
                .get_result::<i64>(conn)
                .unwrap() == 0
        ) {
            diesel::insert_into(game_players::table)
                .values(GamePlayersData {
                    uuid: uuid::Uuid::new_v4().to_string(),
                    user_uuid: Some(p.uuid.to_string()),
                    game_uuid: Some(game_obj.uuid.to_string()),
                })
                .execute(conn)
                .expect("Error saving new game player");
        }
    }
}

pub fn list(conn: &SqliteConnection) -> Vec<GameObj> {

    // TODO paginate
    let page_size = 100;
    let page_id = 0;

    let mut ret = Vec::<GameObj>::new();

    let game_results = games::dsl::games
        .limit(page_size)
        .offset(page_size*page_id)
        .load::<GameData>(conn)
        .expect("Error loading games");

    for g in game_results {
        let players_results = game_players::dsl::game_players
            .filter(game_players::dsl::game_uuid.eq(&g.uuid))
            .load::<GamePlayersData>(conn)
            .expect("Error loading games");
        let mut players = Vec::<PlayerObj>::new();
        for pg in players_results {
            let user = users::dsl::users
                .filter(users::dsl::uuid.eq(pg.user_uuid.unwrap()))
                .load::<UserData>(conn)
                .expect("Error loading games");
            if user.len() != 0 {
                panic!("oh god...");
            }

            let p = PlayerObj {
                uuid: uuid::Uuid::parse_str(&user[0].uuid).unwrap(),
                username: user[0].username.clone(),
            };
            players.push(p);
        }

        ret.push(GameObj {
            uuid: uuid::Uuid::parse_str(&g.uuid).unwrap(),
            max_players_count: g.max_players_count,
            creator_uuid: uuid::Uuid::parse_str(&g.creator_uuid.unwrap()).unwrap(),
            players: players,
        });
    }

    ret
}
