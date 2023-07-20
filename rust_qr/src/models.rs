use super::schema::urls;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = urls)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Url {
    pub id: i32,
    pub small_url: String,
    pub long_url: String,
    pub created_at: Option<String>,
    pub delete_at: Option<String>,
    pub uses: Option<i32>,
}


#[derive(Insertable)]
#[diesel(table_name = urls)]
pub struct NewUrl<'a> {
    pub small_url: &'a str,
    pub long_url: &'a str,
}