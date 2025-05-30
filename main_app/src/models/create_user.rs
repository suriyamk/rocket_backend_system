use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct CreateUser<'a> {
    userName: &'a str,
    companyId: i32,
    isDeleted: bool,
}
