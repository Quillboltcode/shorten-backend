use utoipa::OpenApi;

use crate::models::{
    ApiResponse, Claims, LoginRequest, LoginResponse, RedirectResponse, RefreshTokenRequest,
    ShortenRequest, ShortenResponse,
};

// Define our API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health::health,
        crate::handlers::shortener::shorten_url,
        crate::handlers::redirect::redirect_url,
        crate::handlers::user::register_user,
        crate::handlers::user::get_user,
        crate::handlers::user::update_user,
        crate::handlers::user::delete_user,
        crate::handlers::user::change_password,
        crate::handlers::auth::login,
        crate::handlers::auth::validate_token,
        crate::handlers::auth::refresh_token,
        crate::handlers::auth::logout

    ),
    components(
        schemas(
            ShortenRequest,
            ShortenResponse,
            RedirectResponse,
            LoginRequest,
            LoginResponse,
            RefreshTokenRequest,
            Claims,
            ApiResponse<ShortenResponse>,
            ApiResponse<RedirectResponse>,
            ApiResponse<LoginResponse>,
            ApiResponse<()>
        )
    ),
    tags(
        (name = "api-gateway", description = "API Gateway endpoints")
    )
)]
pub struct ApiDoc;