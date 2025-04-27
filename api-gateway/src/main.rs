mod api_doc;
mod app_state;
mod auth;
mod cache;
mod handlers;
mod models;
mod helpers;

use api_doc::ApiDoc;
use app_state::AppState;
use axum::{routing::{get, post, put, delete}, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Create shared state
    let state = Arc::new(AppState::new());
    
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Build our API documentation
    let openapi = ApiDoc::openapi();
    
    // Configure routes
    let app = Router::new()
        // Core funtional proxied 
        .route("/health", get(handlers::health))
        .route("/shorten", post(handlers::shorten_url))
        .route("/r/:shortcode", get(handlers::redirect_url))
        // User service proxied endpoints
        .route("/users", get(handlers::get_user))
        .route("/users", post(handlers::register_user))
        .route("/users/:user_id", get(handlers::get_user))
        .route("/users/:user_id", put(handlers::update_user))
        .route("/users/:user_id", delete(handlers::delete_user))
        .route("/users/:user_id/password", put(handlers::change_password))
        // Auth endpoints
        .route("/auth/login", post(handlers::login))
        .route("/auth/logout", post(handlers::logout))
        .route("/auth/refresh", post(handlers::refresh_token))
        .route("/auth/validate-token", post(handlers::validate_token))
        .route(
            "/api-docs/openapi.json",
            get({
                let openapi = openapi.clone();
                move || async move { axum::Json(openapi) }
            }),
        )
        .layer(cors)
        .with_state(state);
    
    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8500").await.unwrap();
    tracing::info!("API Gateway listening on http://0.0.0.0:8500");
    axum::serve(listener, app).await.unwrap();
}