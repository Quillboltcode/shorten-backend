use diesel::{Queryable, Selectable};
use crate::schema::url_mapping;
#[allow(dead_code)]
#[derive(Queryable,Selectable)]
#[diesel(table_name = url_mapping)]
pub struct ShortUrl {
    pub short_url: String,
    pub alias: Option<String>,
    pub long_url: String,
    pub creation_date: chrono::NaiveDateTime,
    pub expiration_date: Option<chrono::NaiveDateTime>,
    pub user_id: Option<i32>,
    pub click_count: i32,
}