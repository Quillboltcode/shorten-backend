use std::sync::Arc;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use common::db::{init_pool,DbPool};

mod rabbitmq;
mod routes;
mod cache;
mod models;
mod schema;

#[derive(Clone)]
struct AppState {
    db_pool: DbPool,
    cache: Arc<cache::RedisCache>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _rabbitmq_url = std::env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");

    let db_pool = init_pool(&database_url);
    let cache = Arc::new(cache::RedisCache::new(&redis_url).await.unwrap());

    let redis_client = redis::Client::open(redis_url).expect("Failed to connect to Redis");
    rabbitmq::listen_for_updates(redis_client).await;

    let state = Arc::new(AppState {db_pool,cache});

    let app = Router::new()
        .route("/:short_code", get(routes::redirect))
        .with_state(state.clone()); // Pass database pool

    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
