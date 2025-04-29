// handlers/shortener.rs
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::app_state::AppState;
use crate::models::{ApiResponse, ShortenRequest, ShortenResponse};

// Shortener service endpoint
#[utoipa::path(
    post,
    path = "/shorten",
    tag = "api-gateway",
    request_body = ShortenRequest,
    responses(
        (status = 200, description = "URL shortened successfully", body = ShortenResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn shorten_url(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ShortenRequest>,
) -> impl IntoResponse {
    // Validate token using the user service


    // Forward request to shortener service
    match state
        .shortener_client
        .post("http://shortener-service:8080/shorten")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<ShortenResponse>().await {
                    Ok(data) => (
                        StatusCode::OK,
                        Json(ApiResponse {
                            success: true,
                            data: Some(data),
                            message: None,
                        }),
                    ),
                    Err(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse {
                            success: false,
                            data: None,
                            message: Some("Failed to parse shortener service response".to_string()),
                        }),
                    ),
                }
            } else {
                (
                    StatusCode::from_u16(response.status().as_u16())
                        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: Some("Shortener service error".to_string()),
                    }),
                )
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to connect to shortener service".to_string()),
            }),
        ),
    }
}