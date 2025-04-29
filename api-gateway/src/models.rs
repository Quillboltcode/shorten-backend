use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ShortenRequest {
    pub long_url: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ShortenResponse {
    pub short_code: String,
    pub short_url: String,
    pub expiration_time: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RedirectResponse {
    pub short_url: String,
    pub alias: Option<String>,
    pub long_url: String,
}

// User related models
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserRegistrationRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ValidateTokenResponse {
    pub valid: bool,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub token_type: String,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}