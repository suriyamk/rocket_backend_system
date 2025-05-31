use diesel::Insertable;
use crate::schema::schema::user;

#[derive(Insertable)]
#[diesel(table_name = user)]
struct Company {
    companyId: i32,
    isDeleted: bool,
}