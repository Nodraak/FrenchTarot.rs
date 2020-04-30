use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use uuid::Uuid;

use tarot_lib::game::game::Game as GameObj;
use tarot_lib::player::Player as PlayerObj;

use crate::db::models::Game as GameData;
use crate::db::models::GamePlayers as GamePlayersData;
use crate::db::models::User as UserData;
use crate::db::schema::{games, game_players, users};


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
                uuid: Uuid::new_v4().to_string(),
                user_uuid: Some(p.uuid.to_string()),
                game_uuid: Some(game_obj.uuid.to_string()),
            })
            .execute(conn)
            .expect("Error saving new game player");
    }
}

pub fn get(conn: &SqliteConnection, uuid: Uuid) -> GameObj {

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
            uuid: Uuid::parse_str(&gp.uuid).unwrap(),
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
        uuid: Uuid::parse_str(&results[0].uuid).unwrap(),
        max_players_count: results[0].max_players_count,
        creator_uuid: Uuid::parse_str(&results[0].creator_uuid.as_ref().unwrap()).unwrap(),
        players: players,
    };

    g
}

pub fn list(conn: &SqliteConnection) -> Vec<GameObj> {
    use crate::db::schema::games::dsl::games;
    use crate::db::schema::game_players::dsl::{game_players, game_uuid};
    use crate::db::schema::users::dsl::{users, uuid};

    // TODO paginate
    let page_size = 100;
    let page_id = 0;

    let mut ret = Vec::<GameObj>::new();

    let game_results = games
        .limit(page_size)
        .offset(page_size*page_id)
        .load::<GameData>(conn)
        .expect("Error loading games");

    for g in game_results {
        let players_results = game_players
            .filter(game_uuid.eq(&g.uuid))
            .load::<GamePlayersData>(conn)
            .expect("Error loading games");
        let mut players = Vec::<PlayerObj>::new();
        for pg in players_results {
            let user = users
                .filter(uuid.eq(pg.user_uuid.unwrap()))
                .load::<UserData>(conn)
                .expect("Error loading games");
            if user.len() != 0 {
                panic!("oh god...");
            }

            let p = PlayerObj {
                uuid: Uuid::parse_str(&user[0].uuid).unwrap(),
                username: user[0].username.clone(),
            };
            players.push(p);
        }

        ret.push(GameObj {
            uuid: Uuid::parse_str(&g.uuid).unwrap(),
            max_players_count: g.max_players_count,
            creator_uuid: Uuid::parse_str(&g.creator_uuid.unwrap()).unwrap(),
            players: players,
        });
    }

    ret
}
