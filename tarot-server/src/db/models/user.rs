use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use serde::Serialize;
use uuid;

use crate::db::models::user::User as UserData;
use crate::db::schema::users;
use crate::db::utils;
use crate::routes::utils::User as UserObj;


#[derive(Insertable, Queryable)]
#[derive(Serialize)]
#[table_name="users"]
pub struct User {
    pub uuid: utils::Uuid,
    pub username: String,
}


pub fn create(conn: &SqliteConnection, user_obj: &UserObj) {

    diesel::insert_into(users::table)
        .values(UserData {
            uuid: user_obj.uuid.to_string(),
            username: user_obj.username.clone(),
        })
        .execute(conn)
        .expect("Error saving new user");
}

pub fn get(conn: &SqliteConnection, uuid: uuid::Uuid) -> UserObj {
    let results = users::dsl::users
        .filter(users::dsl::uuid.eq(uuid.to_string()))
        .load::<UserData>(conn)
        .expect("Error loading games");

    if results.len() != 1 {
        panic!("oh god - user get");
    }

    UserObj {
        uuid: uuid::Uuid::parse_str(&results[0].uuid).unwrap(),
        username: results[0].username.clone(),
    }
}

pub fn update(conn: &SqliteConnection, uuid: uuid::Uuid, user_obj: UserObj) {
    diesel::update(users::dsl::users.find(uuid.to_string()))
        .set(users::dsl::username.eq(user_obj.username))
        .execute(conn)
        .expect(&format!("Error updating user {}", uuid));
}
