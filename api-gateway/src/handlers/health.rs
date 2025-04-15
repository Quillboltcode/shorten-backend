// handlers/health.rs
use axum::response::IntoResponse;

// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    tag = "api-gateway",
    responses(
        (status = 200, description = "API Gateway is healthy")
    ),
)]
pub async fn health() -> impl IntoResponse {
    "API Gateway is healthy"
}