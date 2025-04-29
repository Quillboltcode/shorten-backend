use reqwest::Client;
use std::sync::Arc;
use crate::cache::TokenCache;

pub struct AppState {
    pub shortener_client: Client,
    pub redirect_client: Client,
    pub user_service_client: Client,
    pub jwt_secret: String,
    pub token_cache: Arc<TokenCache>,
    pub user_service_url: String,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            shortener_client: Client::new(),
            redirect_client: Client::new(),
            user_service_client: Client::new(),
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key".to_string()),
            token_cache: Arc::new(TokenCache::new()),
            user_service_url: std::env::var("USER_SERVICE_URL").unwrap_or_else(|_| "http://user-service:8082/api".to_string()),
        }
    }
}