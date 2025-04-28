use axum::{
    extract::{Query, State},
    Json,
    http::StatusCode,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::{DateTime, Utc};
use crate::models::url::UrlMappingModel;
use crate::schema::url_mapping::dsl::*;
use common::db::DbPool;

#[derive(Deserialize)]
struct QueryParams {
    user_id: i32,
}

#[derive(Serialize)]
pub struct UrlInfoResponse {
    pub short_code: String,
    pub short_url: String,
    pub long_url: String,
    pub alias: Option<String>,
    pub creation_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub click_count: i32,
}

impl From<UrlMappingModel> for UrlInfoResponse {
    fn from(mapping: UrlMappingModel) -> Self {
        let short_code = mapping.short_url.clone();
        let short_url = format!("http://localhost:8080/{}", short_code);

        UrlInfoResponse {
            short_code,
            short_url ,
            long_url: mapping.long_url,
            alias: mapping.alias,
            creation_date: DateTime::<Utc>::from_naive_utc_and_offset(mapping.creation_date, Utc),
            expiration_date: mapping.expiration_date.map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)),
            click_count: mapping.click_count,
        }
    }
}

/// Retrieve all URL mappings for a given user ID.
pub async fn get_urls_by_user_id(
    Query(params): Query<QueryParams>,
    State(pool): State<Arc<DbPool>>,
) -> Result<Json<Vec<UrlInfoResponse>>, StatusCode> {
    use diesel::RunQueryDsl;

    // Get a database connection from the pool
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Query the database for URL mappings matching the user_id
    let results = url_mapping
        .filter(user_id.eq(params.user_id))
        .load::<UrlMappingModel>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Convert the results into the response format
    let response = results.into_iter().map(UrlInfoResponse::from).collect();

    Ok(Json(response))
}