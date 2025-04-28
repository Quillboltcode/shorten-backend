use axum::{routing::post, Router};
use diesel::prelude::*;
use diesel::PgConnection;
use tracing::info;
use tracing::Level;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::{TraceLayer, DefaultOnResponse, DefaultMakeSpan}; 
use common::logging::init_tracing;
use common::db::{init_pool,run_migrations};

mod models;
mod routes;
mod schema;
mod rabbitmq;
mod hashcode;



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url).expect("Failed to connect to DB");

    print!("Connected to database");

    run_migrations(&mut conn);
    info!("✅ Migrations completed");
    let db_pool = init_pool(&database_url);

    let state = Arc::new(db_pool);
    let app = Router::new()
        .route("/lookup/user/", post(routes::lookup::get_urls_by_user_id))
    
        .route("/shorten", post(routes::urlshort::shorten_url))
        .layer(
            TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
        )
        .with_state(state.clone());

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
