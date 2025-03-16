use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::{url_mapping, account};


#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = url_mapping)]
pub struct UrlMappingModel  {
    pub short_url: String,
    pub alias: Option<String>,
    pub long_url: String,
    pub creation_date: NaiveDateTime,
    pub expiration_date: Option<NaiveDateTime>,
    pub user_id: Option<i32>,  
    pub click_count: i32,
}