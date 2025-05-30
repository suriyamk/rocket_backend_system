use diesel::prelude::*;
use crate::schema::schema::user;
use diesel::sql_types::Date;

#[derive(Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct User {
    userId: i32,
    userName: String,
    companyId: i32,
    isDeleted: bool,
    createdDate: Date,
    modifiedDate: Date,
}
