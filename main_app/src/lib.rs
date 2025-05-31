pub mod schema;
pub mod models;
#[macro_use] extern crate rocket;

use diesel::prelude::*;
use crate::models::user::User;
use crate::models::create_user::CreateUser;

pub fn create_user(conn: &mut SqliteConnection, &user_model: &User) -> User {
    use crate::schema::schema::user;
    let user_obj = CreateUser { userName: &user_model.userName, companyId: user_model.companyId, isDeleted: false };
    diesel::insert_into(user::table)
        .values(user_obj)
        .execute(conn)
        .expect("Error while saving user data");

    user::table
        .order(user::userId.desc())
        .first(conn)
        .expect("Error while getting user")
}