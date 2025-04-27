// handlers/user.rs
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::app_state::AppState;
use crate::auth::validate_token;
use crate::models::{ApiResponse, ChangePasswordRequest, UpdateUserRequest, UserRegistrationRequest};
use crate::helpers::convert_axum_to_reqwest_headers;

// Endpoints from user-service:
///    GET  /users (Get All Users) 
//     POST /users (Register User)
//     GET /users/{userId} (Get User Profile)
//     PUT /users/{userId} (Update User Profile)
//     PUT /users/{userId}/password (Change User Password)
//     DELETE /users/{userId} (Delete User Account)

// Get Alll Users endpoint
#[utoipa::path(
    get,
    path = "/users",
    tag = "api-gateway",
    responses(
        (status = 200, description = "Users retrieved successfully", body = [UserResponse]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_users(
    State(state): State<Arc<AppState>>,
    // headers: HeaderMap,
) -> impl IntoResponse {
    // Forward get all users request to user service
    match state
        .user_service_client
        .get(format!("{}/users", state.user_service_url))
        // .headers(headers)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            match response.json::<serde_json::Value>().await {
                Ok(data) => (
                    StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    Json(data),
                ),
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




// User registration endpoint
#[utoipa::path(
    post,
    path = "/users",
    tag = "api-gateway",
    request_body = UserRegistrationRequest,
    responses(
        (status = 201, description = "User registered successfully", body = UserResponse),
        (status = 400, description = "Invalid request data"),
        (status = 409, description = "Email already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UserRegistrationRequest>,
) -> impl IntoResponse {
    // Forward registration request to user service
    match state
        .user_service_client
        .post(format!("{}/users", state.user_service_url))
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            match response.json::<serde_json::Value>().await {
                Ok(data) => (
                    StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    Json(data),
                ),
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

// Get user endpoint
#[utoipa::path(
    get,
    path = "/users/{user_id}",
    tag = "api-gateway",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User information", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
)-> impl IntoResponse {
    // Validate token using the user service
    if let Err(err) = validate_token(&headers, &state, "access").await {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "success": false,
                "message": err
            })),
        );
    }

    // Forward request to user service
    match state
        .user_service_client
        .get(format!("{}/users/{}", state.user_service_url, user_id))
        .headers(
            convert_axum_to_reqwest_headers(&headers)
        )
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            match response.json::<serde_json::Value>().await {
                Ok(data) => (
                    StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    Json(data),
                ),
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

// Update user endpoint
#[utoipa::path(
    put,
    path = "/users/{user_id}",
    tag = "api-gateway",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Cannot update other users"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
    Json(request): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    // Validate token using the user service
    if let Err(err) = validate_token(&headers, &state, "access").await {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "success": false,
                "message": err
            })),
        );
    }

    // Forward request to user service
    match state
        .user_service_client
        .put(format!("{}/users/{}", state.user_service_url, user_id))
        .headers(
            convert_axum_to_reqwest_headers(&headers)
        )
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            match response.json::<serde_json::Value>().await {
                Ok(data) => (
                    StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    Json(data),
                ),
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

// Delete user endpoint
#[utoipa::path(
    delete,
    path = "/users/{user_id}",
    tag = "api-gateway",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Cannot delete other users"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    // Validate token using the user service
    if let Err(err) = validate_token(&headers, &state, "access").await {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "success": false,
                "message": err
            })),
        );
    }

    // Forward request to user service
    match state
        .user_service_client
        .delete(format!("{}/users/{}", state.user_service_url, user_id))
        .headers(
            convert_axum_to_reqwest_headers(&headers)
                
        )
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            match response.json::<serde_json::Value>().await {
                Ok(data) => (
                    StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    Json(data),
                ),
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

// Change password endpoint
#[utoipa::path(
    post,
    path = "/users/{user_id}/change-password",
    tag = "api-gateway",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Cannot change other users' password"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn change_password(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
    Json(request): Json<ChangePasswordRequest>,
) -> impl IntoResponse {
    // Validate token using the user service
    if let Err(err) = validate_token(&headers, &state, "access").await {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(err),
            }),
        );
    }

    // Forward request to user service
    match state
        .user_service_client
        .post(format!("{}/users/{}/change-password", state.user_service_url, user_id))
        .headers(
            convert_axum_to_reqwest_headers(&headers)
                
            )
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            match response.json::<serde_json::Value>().await {
                Ok(data) => (
                    StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    Json(ApiResponse::<()> {
                        success: true,
                        data: None,
                        message: Some(data["message"].as_str().unwrap_or("Password changed successfully").to_string()),
                    }),
                ),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<()> {
                        success: false,
                        data: None,
                        message: Some("Failed to connect to user service".to_string()),
                    }),
                ),
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to parse user service response".to_string()),
            }),
        ),
    }
}