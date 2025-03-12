use axum::{extract::{Path, State}, response::Redirect};
use diesel::prelude::*;
use std::sync::Arc;
use crate::AppState;
use crate::{models::ShortUrl, schema::short_urls::dsl::*};


pub async fn redirect(
    Path(other_short_code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Redirect {
    if let Some(cached_url) = state.cache.get(&other_short_code).await {
        return Redirect::temporary(&cached_url);
    }

    let conn = &mut state.db_pool.get().expect("Failed to get DB connection");

    if let Ok(Some(record)) = short_urls
        .filter(short_code.eq(&short_code))
        .select((id, short_code, original_url))
        .first::<ShortUrl>(conn)
        .optional()
    {
        let _ = state.cache.set(&record.short_code, &record.original_url, 3600).await;
        Redirect::temporary(&record.original_url)
    } else {
        Redirect::temporary("https://example.com/404")
    }
}
