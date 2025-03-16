use axum::{extract::State, Json, http::StatusCode};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::{DateTime, Duration, Utc};
use url::Url;

use common::db::DbPool;
use crate::models::url::UrlMappingModel;
use crate::{rabbitmq::publish_to_queue, schema::url_mapping::dsl::*};
use crate::hashcode::generate_short_code;

#[derive(Deserialize, Serialize)]
pub struct ShortenRequest {
    pub long_url: String,
    pub custom_alias: Option<String>,
    pub expiration_time: Option<DateTime<Utc>>,
    pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ShortenResponse {
    pub short_code: String,
    pub short_url: String,
    pub expiration_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
impl From<UrlMappingModel> for ShortenResponse {
    fn from(mapping: UrlMappingModel) -> Self {
        let short_code = mapping.short_url.clone();
        let other_short_url = mapping
            .alias
            .clone()
            .unwrap_or_else(|| mapping.short_url.clone()); // Correct reference

        ShortenResponse {
            short_code,
            short_url: format!("http://localhost:8080/{}", other_short_url), // Full short URL
            expiration_time: mapping
                .expiration_date
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(||Utc::now() +  Duration::days(30)), // Default to now if expiration is missing + 30 days
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(mapping.creation_date, Utc),
        }
    }
}
// Validate the URL
pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}   

/// Shorten a URL and store it in the database.
pub async fn shorten_url(
    State(pool): State<Arc<DbPool>>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, StatusCode> {
    
    // Validate URL
    if !is_valid_url(&payload.long_url) {
        return Ok(Json(ShortenResponse {
            short_code: "invalid".to_string(),
            short_url: "https://example.com/400".to_string(), // Bad Request
            created_at: Utc::now(), // Default to now
            expiration_time: Utc::now() + Duration::days(30), // Default expiration
        }));
    }

    
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;


    // Fetch existing short codes to prevent collisions
    let existing_codes: Vec<String> = url_mapping
        .select(short_url)
        .load::<String>(&mut conn)
        .unwrap_or_default();

    let short_code_value = generate_short_code(&payload.long_url, &existing_codes.into_iter().collect());

    let new_entry = UrlMappingModel {
        short_url: short_code_value.clone(), // Use the generated short code
        alias: None, // No alias by default; can be set later if needed
        long_url: payload.long_url.clone(),
        creation_date: Utc::now().naive_utc(),
        expiration_date: Some(Utc::now().naive_utc() + Duration::days(30)), // Set expiration 30 days later
        user_id: None, // Anonymous by default; can be linked to a user if authenticated
        click_count: 0, // Initialize click count to 0
    };

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        diesel::insert_into(url_mapping)
            .values(&new_entry)
            .execute(conn)?;

        Ok(())
    }).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Publish to RabbitMQ (async, but errors don't break transaction)
    if let Err(err) = publish_to_queue(&short_code_value, &payload.long_url).await {
        eprintln!("⚠️ Failed to publish to RabbitMQ: {}", err);
    }

    Ok(Json(ShortenResponse {
        short_code: short_code_value.clone(),
        short_url: format!("http://localhost:8081/{}", short_code_value),
        created_at: Utc::now(),
        expiration_time: Utc::now() + Duration::days(30), // Default expiration
    }))
}
