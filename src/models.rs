use diesel::prelude::*;
use crate::schema; // Correct import statement

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}