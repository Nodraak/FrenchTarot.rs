use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;

use super::models::GameNew as GameData;
use super::models::GamePlayersNew as GamePlayersData;
use super::models::UserNew as UserData;
use super::schema::{users, games, game_players};

use tarot_lib::game::Game as GameObj;
use tarot_lib::player::Player as PlayerObj;


pub fn game_create(conn: &SqliteConnection, game_obj: GameObj) {
    let c_pk = match game_obj.creator {
        None => { 42 },
        Some(c) =>  {
            c.pk
        }
    };

    let game_data = GameData {
        max_players_count: game_obj.max_players_count,
        creator_pk: Some(c_pk),
    };

    let r = diesel::insert_into(games::table)
        .values(&game_data)
        .execute(conn)
        .expect("Error saving new post");

    println!("Inserted {}", r);

/*

diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")


    for p in game.players {
        let g = GamePlayersData {
            user_pk: XX,
            game_pk: XX,
        };

        diesel::insert_into(games::table)
            .values(&g)
            .execute(conn)
            .expect("Error saving new post");

    }
*/
}
