use axum::{routing::post, Router};
use diesel::prelude::*;
use diesel::PgConnection;
use tracing::Level;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::{TraceLayer, DefaultOnResponse, DefaultMakeSpan}; 

mod db;
mod model;
mod routes;
mod schema;
mod rabbitmq;



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").with_target(false).init();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _conn = PgConnection::establish(&database_url).expect("Failed to connect to DB");

    print!("Connected to database");

    let db_pool = db::init_pool(&database_url);

    let state = Arc::new(db_pool);

    let app = Router::new()
        .route("/shorten", post(routes::shorten_url))
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
