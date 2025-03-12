use axum::{extract::State, Json};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::{db::DbPool, model::ShortUrl, rabbitmq::publish_to_queue, schema::short_urls::dsl::*};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub original_url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_code: String,
    pub short_url: String,
}

pub async fn shorten_url(
    State(pool): State<Arc<DbPool>>,
    Json(payload): Json<ShortenRequest>,
) -> Json<ShortenResponse> {
    let conn = &mut match pool.get() {
        Ok(c) => c,
        Err(_) => return Json(ShortenResponse {
            short_code: "error".to_string(),
            short_url: "https://example.com/500".to_string(),
        }),
    };

    // Generate a unique short code (first 8 chars of UUID)
    let short_code_value = Uuid::new_v4().to_string()[..8].to_string();

    let new_entry = ShortUrl {
        id: Uuid::new_v4(), // Ensure UUID is used
        short_code: short_code_value.clone(),
        original_url: payload.original_url.clone(),
        created_at: Utc::now().naive_utc(),
    };

    if let Err(_) = diesel::insert_into(short_urls)
        .values(&new_entry)
        .execute(conn)
    {
        return Json(ShortenResponse {
            short_code: "error".to_string(),
            short_url: "https://example.com/500".to_string(),
        });
    }

        // Publish the short URL to RabbitMQ
    if let Err(err) = publish_to_queue(&short_code_value, &payload.original_url).await {
            eprintln!("Failed to publish to RabbitMQ: {}", err);
    }

    let response = ShortenResponse {
        short_code: short_code_value.clone(),
        short_url: format!("http://localhost:8080/{}", short_code_value),
    };

    Json(response)
}
