use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = user)]
struct Company {
    companyId: i32,
    isDeleted: bool,
}