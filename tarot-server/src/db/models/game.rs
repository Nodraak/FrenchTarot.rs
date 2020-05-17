use std::cmp::PartialEq;

use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use serde::Serialize;
use uuid;

use tarot_lib::game::game::Game as GameObj;
use tarot_lib::player::Player as PlayerObj;

use crate::db::models::user::User as UserData;
use crate::db::schema::{games, game_players, users};
use crate::db::utils;


#[derive(Debug, PartialEq)]
#[derive(AsChangeset, Insertable, Queryable)]
#[derive(Serialize)]
#[table_name="games"]
#[primary_key(uuid)]
pub struct Game {
    pub uuid: utils::Uuid,
    pub max_players_count: i32,
    pub creator_uuid: Option<utils::Uuid>,  // User.Uuid
}

#[derive(Debug, PartialEq)]
#[derive(AsChangeset, Insertable, Queryable)]
#[derive(Serialize)]
#[table_name="game_players"]
#[primary_key(uuid)]
pub struct GamePlayers {
    pub uuid: utils::Uuid,
    pub user_uuid: Option<utils::Uuid>,  // User.uuid
    pub game_uuid: Option<utils::Uuid>,  // Game.uuid
}

impl Game {
    pub fn from_GameObj(conn: &SqliteConnection, game_obj: &GameObj) -> (Game, Vec<GamePlayers>) {

        // convert GameObj to Game

        let game_db = Game {
            uuid: game_obj.uuid.to_string(),
            max_players_count: game_obj.max_players_count,
            creator_uuid: Some(game_obj.creator_uuid.to_string()),
        };

        // load/create GamePlayers

        let mut game_players_db = Vec::new();

        for player_obj in &game_obj.players {

            let result = users::dsl::users
                .find(&player_obj.uuid.to_string())
                .get_result::<UserData>(conn);

            let player_uuid = match result {
                Err(_) => { uuid::Uuid::new_v4().to_string() },
                Ok(player_db) => { player_db.uuid },
            };

            game_players_db.push(GamePlayers {
                uuid: player_uuid,
                user_uuid: Some(player_obj.uuid.to_string()),
                game_uuid: Some(game_db.uuid.clone()),
            });
        }

        // return

        (game_db, game_players_db)
    }

    pub fn to_GameObj(&self, conn: &SqliteConnection, game_players: &Vec<GamePlayers>) -> GameObj {

        // convert GamePlayers to Players

        let mut players_obj: Vec<PlayerObj> = Vec::new();

        for game_player_db in game_players {
            // load player
            let player_db: UserData = users::dsl::users
                .find(&game_player_db.uuid)
                .get_result(conn)
                .expect("Could not load User");

            players_obj.push(PlayerObj {
                uuid: uuid::Uuid::parse_str(&game_player_db.uuid).unwrap(),
                username: player_db.username.clone(),
            });
        }

        // convert Game to GameObj

        let g = GameObj {
            uuid: uuid::Uuid::parse_str(&self.uuid).unwrap(),
            max_players_count: self.max_players_count,
            creator_uuid: uuid::Uuid::parse_str(&self.creator_uuid.clone().unwrap()).unwrap(),
            players: players_obj,
        };

        // return

        g
    }
}


pub fn create(conn: &SqliteConnection, game_obj: GameObj) {

    // convert
    let (g, gps) = Game::from_GameObj(conn, &game_obj);

    // insert game

    diesel::insert_into(games::table)
        .values(g)
        .execute(conn)
        .expect("Could not save new Game");

    // insert game players

    for gp in gps {
        diesel::insert_into(game_players::table)
            .values(gp)
            .execute(conn)
            .expect("Could not save new GamePlayers");
    }
}

pub fn get(conn: &SqliteConnection, uuid: uuid::Uuid) -> GameObj {

    // get game_players

    let results: Vec<GamePlayers> = game_players::dsl::game_players
        .filter(game_players::dsl::game_uuid.eq(uuid.to_string()))
        .load(conn)
        .expect("Could not load GamePlayers");

    // get game

    let result: Game = games::dsl::games
        .find(uuid.to_string())
        .get_result(conn)
        .expect("Could not load Game");

    // convert and return

    result.to_GameObj(conn, &results)
}

pub fn update(conn: &SqliteConnection, uuid: uuid::Uuid, game_obj: GameObj) {

    let (game_db, game_players_db_new) = Game::from_GameObj(conn, &game_obj);

    let game_players_db_old = game_players::dsl::game_players
        .filter(game_players::dsl::game_uuid.eq(game_obj.uuid.to_string()))
        .load::<GamePlayers>(conn)
        .expect("Could not load GamePlayers");

    // update game

    diesel::update(games::table).set(game_db);

    // update gp: delete old gp

    for gp in &game_players_db_old {
        if game_players_db_new.contains(&gp) == false {
            diesel::delete(game_players::dsl::game_players.find(&gp.uuid))
                .execute(conn)
                .expect("Could not delete GamePlayers");
        }
    }

    // update gp: create new gp

    for gp in &game_players_db_new {
        if game_players_db_old.contains(&gp) == false {
            diesel::insert_into(game_players::table)
                .values(gp)
                .execute(conn)
                .expect("Could not save new GamePlayers");
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
        .load::<Game>(conn)
        .expect("Could not load Game");

    for g in game_results {

        let results = game_players::dsl::game_players
            .filter(game_players::dsl::game_uuid.eq(&g.uuid))
            .load::<GamePlayers>(conn)
            .expect("Coult not load GamePlayers");

        ret.push(g.to_GameObj(conn, &results));
    }

    ret
}
