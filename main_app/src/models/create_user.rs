use diesel::Insertable;
use crate::schema::schema::user;

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct CreateUser<'a> {
    pub userName: &'a str,
    pub companyId: i32,
    pub isDeleted: bool,
}
