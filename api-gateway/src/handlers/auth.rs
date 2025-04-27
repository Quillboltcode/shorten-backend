// handlers/auth.rs
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::app_state::AppState;
use crate::models::{
    ApiResponse, LoginRequest, RefreshTokenRequest,
    ValidateTokenRequest, ValidateTokenResponse,
};

//     POST /auth/login (Login and Generate Token)
//     POST /auth/logout (Logout and Invalidate Token)
//     POST /auth/validate-token (Validate Token)

// Login endpoint
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "api-gateway",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    // Forward login request to user service
    match state
        .user_service_client
        .post(format!("{}/auth/login", state.user_service_url))
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            match response.json::<serde_json::Value>().await {
                Ok(data) => {
                    // If login was successful, store tokens in cache
                    if status.is_success() {
                        if let (Some(access_token), Some(user_id)) = (
                            data["data"]["access_token"].as_str(),
                            data["data"]["user_id"].as_str(),
                        ) {
                            // Store access token in cache with default expiration (15min)
                            let exp_time = chrono::Utc::now() + chrono::Duration::minutes(15);
                            state.token_cache.store_token(access_token, user_id.to_string(), exp_time);
                        }
                    }
                    
                    (
                        StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                        Json(data),
                    )
                },
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "success": false,
                        "message": "Failed to parse user service response"
                    })),
                ),
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "message": "Failed to connect to user service"
            })),
        ),
    }
}

// Logout endpoint
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "api-gateway",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Extract token from Authorization header
    let auth_header = match headers.get("Authorization") {
        Some(header) => match header.to_str() {
            Ok(header_str) => header_str,
            Err(_) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse::<()> {
                        success: false,
                        data: None,
                        message: Some("Invalid authorization header".to_string()),
                    }),
                );
            }
        },
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<()> {
                    success: false,
                    data: None,
                    message: Some("No authorization token provided".to_string()),
                }),
            );
        }
    };

    // Extract the token
    if !auth_header.starts_with("Bearer ") {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Invalid authorization format".to_string()),
            }),
        );
    }
    
    let token = &auth_header[7..]; // Skip "Bearer "
    
    // Invalidate token in cache
    state.token_cache.invalidate_token(token);
    
    // Forward logout request to user service
    match state
        .user_service_client
        .post(format!("{}/auth/logout", state.user_service_url))
        .headers({
            let mut reqwest_headers = reqwest::header::HeaderMap::new();
    
            // Copy over the headers you need
            if let Some(auth_header) = headers.get("Authorization") {
                reqwest_headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(auth_header.to_str().unwrap_or_default()).expect("Failed to parse auth header"),
                );
            }
            reqwest_headers
        })
        .send()
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some("Logout successful".to_string()),
            }),
        ),
        Err(_) => (
            StatusCode::OK, // Still return OK to client even if user service fails
            Json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some("Logout successful".to_string()),
            }),
        ),
    }

}
// Refresh token endpoint
#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "api-gateway",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = LoginResponse),
        (status = 401, description = "Invalid or expired refresh token"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    // Forward refresh token request to user service
    match state
        .user_service_client
        .post(format!("{}/auth/refresh", state.user_service_url))
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            match response.json::<serde_json::Value>().await {
                Ok(data) => {
                    // If refresh was successful, update cache with new token
                    if status.is_success() {
                        if let (Some(access_token), Some(user_id)) = (
                            data["data"]["access_token"].as_str(),
                            data["data"]["user_id"].as_str(),
                        ) {
                            // Store new access token in cache
                            let exp_time = chrono::Utc::now() + chrono::Duration::minutes(15);
                            state.token_cache.store_token(access_token, user_id.to_string(), exp_time);
                        }
                    }
                    
                    (
                        StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                        Json(data),
                    )
                },
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "success": false,
                        "message": "Failed to parse user service response"
                    })),
                ),
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "message": "Failed to connect to user service"
            })),
        ),
    }
}



// Validate token endpoint
#[utoipa::path(
    post,
    path = "/auth/validate-token",
    tag = "api-gateway",
    request_body = ValidateTokenRequest,
    responses(
        (status = 200, description = "Token validation result", body = ValidateTokenResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn validate_token(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ValidateTokenRequest>,
) -> impl IntoResponse {
    // Check if token is in cache
    if let Some(token_info) = state.token_cache.get_token(&request.token) {
        return (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(ValidateTokenResponse {
                    valid: true,
                    user_id: Some(token_info.user_id),
                }),
                message: None,
            }),
        );
    }
   
    // Token not in cache, forward validation request to user service
    match state
        .user_service_client
        .post(format!("{}/auth/validate-token", state.user_service_url))
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            match response.json::<serde_json::Value>().await {
                Ok(data) => {
                    // If token is valid, add to cache
                    if let (Some(valid), Some(user_id)) = (
                        data["data"]["valid"].as_bool(),
                        data["data"]["user_id"].as_str(),
                    ) {
                        if valid {
                            // Add token to cache with default expiration
                            let exp_time = chrono::Utc::now() + chrono::Duration::minutes(15);
                            state.token_cache.store_token(&request.token, user_id.to_string(), exp_time);
                            
                            return (
                                StatusCode::OK,
                                Json(ApiResponse {
                                    success: true,
                                    data: Some(ValidateTokenResponse {
                                        valid: true,
                                        user_id: Some(user_id.to_string()),
                                    }),
                                    message: None,
                                }),
                            );
                        }
                    }
                    
                    // Parse the response appropriately
                    let valid = data["data"]["valid"].as_bool().unwrap_or(false);
                    let user_id = data["data"]["user_id"].as_str().map(|s| s.to_string());
                    
                    (
                        StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                        Json(ApiResponse {
                            success: true,
                            data: Some(ValidateTokenResponse {
                                valid,
                                user_id,
                            }),
                            message: None,
                        }),
                    )
                },
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<ValidateTokenResponse> {
                        success: false,
                        data: None,
                        message: Some("Failed to parse user service response".to_string()),
                    }),
                ),
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<ValidateTokenResponse> {
                success: false,
                data: None,
                message: Some("Failed to connect to user service".to_string()),
            }),
        ),
    }
}