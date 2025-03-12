use diesel::prelude::*;

#[derive(Queryable)]
pub struct ShortUrl {
    pub id: i32,
    pub short_code: String,
    pub original_url: String,
}
