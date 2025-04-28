use crate::AppState;
use crate::{models::ShortUrl, schema::url_mapping::dsl::*};
use axum::{
    extract::{Path, State},
    response::{Redirect, IntoResponse, Json},
    http::StatusCode,
};
use diesel::prelude::*;
use std::sync::Arc;


// Custom error response for 404
#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

#[derive(serde::Serialize)]
struct UrlResponse {
    short_url: String,
    alias: Option<String>, // Include the alias field
    long_url: String,
}

pub async fn redirect(
    Path(other_short_code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {

    if let Some(cached_url) = state.cache.get(&other_short_code).await {
        return Redirect::temporary(&cached_url).into_response();
    }

    let conn = &mut state.db_pool.get().expect("Failed to get DB connection");

    let record = url_mapping
        .filter(short_url.eq(&other_short_code))
        .select(ShortUrl::as_select()) // Now works because `Selectable` is derived
        .first::<ShortUrl>(conn)
        .optional();

    match record {
        Ok(Some(record)) => {
            let _ = state.cache.set(&record.short_url, &record.long_url, 3600).await;
                        // Return the URL details as JSON
            let response = UrlResponse {
                short_url: record.short_url,
                alias: record.alias, // Include the alias
                long_url: record.long_url,
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let error_response = ErrorResponse {
                error: "Not Found".to_string(),
                message: format!("No URL found for short code: {}", other_short_code),
            };
            (StatusCode::NOT_FOUND, Json(error_response)).into_response()
        }
        Err(_) => {
            let error_response = ErrorResponse {
                error: "Internal Server Error".to_string(),
                message: "An error occurred while processing your request.".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}
