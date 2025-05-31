use diesel::prelude::*;
use crate::schema::schema::user;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub userId: i32,
    pub userName: String,
    pub companyId: i32,
    pub isDeleted: bool,
    pub createdDate: NaiveDateTime,
    pub modifiedDate: NaiveDateTime,
}
