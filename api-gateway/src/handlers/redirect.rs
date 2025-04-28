// handlers/redirect.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::app_state::AppState;
use crate::models::{ApiResponse, RedirectResponse};

// Redirect service endpoint
#[utoipa::path(
    get,
    path = "/r/{shortcode}",
    tag = "api-gateway",
    params(
        ("shortcode" = String, Path, description = "Shortened URL code")
    ),
    responses(
        (status = 200, description = "Redirect information", body = RedirectResponse),
        (status = 404, description = "Shortcode not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn redirect_url(
    State(state): State<Arc<AppState>>,
    Path(shortcode): Path<String>,
) -> impl IntoResponse {
    // Forward request to redirect service
    match state
        .redirect_client
        .get(format!("http://redirect-service:8081/{}", shortcode))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<RedirectResponse>().await {
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
                        Json(ApiResponse::<RedirectResponse> {
                            success: false,
                            data: None,
                            message: Some("Failed to parse redirect service response".to_string()),
                        }),
                    ),
                }
            } else if response.status().as_u16() == StatusCode::NOT_FOUND.as_u16() {
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::<RedirectResponse> {
                        success: false,
                        data: None,
                        message: Some("Shortcode not found".to_string()),
                    }),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<RedirectResponse> {
                        success: false,
                        data: None,
                        message: Some("Redirect service error".to_string()),
                    }),
                )
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<RedirectResponse> {
                success: false,
                data: None,
                message: Some("Failed to connect to redirect service".to_string()),
            }),
        ),
    }
}