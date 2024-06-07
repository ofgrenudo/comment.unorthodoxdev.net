use diesel::prelude::*;
use super::schema::posts;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = comments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub ip_address: String,
    pub username: String,
    pub comment: String,
    pub timestamp: String,
    pub visible: Bool,
}