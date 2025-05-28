use diesel::prelude::*;
use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
struct User {
    userId: i64,
    userName: String,
    companyId: i64,
    isDeleted: bool,
    createdDate: Date,
    modifiedDate: Date,
}