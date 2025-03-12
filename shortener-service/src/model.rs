use diesel::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::short_urls)]
pub struct ShortUrl {
    #[diesel(sql_type = DieselUuid)]  //  Map to SQL UUID
    pub id: Uuid, 
    pub short_code: String,
    pub original_url: String,
    pub created_at: NaiveDateTime,
}
