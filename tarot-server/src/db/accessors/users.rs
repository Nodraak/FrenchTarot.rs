use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::models::User as UserData;
use crate::db::schema::users::{table, dsl};
use crate::routes::utils::User as UserObj;


pub fn create(conn: &SqliteConnection, user_obj: &UserObj) {

    diesel::insert_into(table)
        .values(UserData {
            uuid: user_obj.uuid.to_string(),
            username: user_obj.username.clone(),
        })
        .execute(conn)
        .expect("Error saving new user");
}

pub fn get(conn: &SqliteConnection, uuid: Uuid) -> UserObj {
    let results = dsl::users
        .filter(dsl::uuid.eq(uuid.to_string()))
        .load::<UserData>(conn)
        .expect("Error loading games");

    if results.len() != 1 {
        panic!("oh god - user get");
    }

    UserObj {
        uuid: Uuid::parse_str(&results[0].uuid).unwrap(),
        username: results[0].username.clone(),
    }
}

pub fn update(conn: &SqliteConnection, uuid: Uuid, user_obj: UserObj) {
    diesel::update(dsl::users.find(uuid.to_string()))
        .set(dsl::username.eq(user_obj.username))
        .execute(conn)
        .expect(&format!("Error updating user {}", uuid));
}
