use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::account;


#[derive(Queryable, Serialize)]
#[diesel(table_name = account)]
pub struct UserModel {
    pub user_id: i32,
    pub email: String,
    pub password_hash : String,
    pub create_at : NaiveDateTime
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = account)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}

