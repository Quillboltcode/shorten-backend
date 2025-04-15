use std::collections::HashMap;
use std::sync::RwLock;
use chrono::{DateTime, Utc};


// Structure to store cached token information
pub struct TokenInfo {
    pub user_id: String,
    pub expires_at: DateTime<Utc>,
    pub is_valid: bool,
}

// Token cache to reduce calls to user service for validation
pub struct TokenCache {
    cache: RwLock<HashMap<String, TokenInfo>>,
}

impl TokenCache {
    pub fn new() -> Self {
        TokenCache {
            cache: RwLock::new(HashMap::new()),
        }
    }

    // Store token info in cache
    pub fn store_token(&self, token: &str, user_id: String, expires_at: DateTime<Utc>) {
        let mut cache = self.cache.write().unwrap();
        cache.insert(token.to_string(), TokenInfo {
            user_id,
            expires_at,
            is_valid: true,
        });
    }

    // Get token info from cache if it exists and is valid
    pub fn get_token(&self, token: &str) -> Option<TokenInfo> {
        let cache = self.cache.read().unwrap();
        if let Some(info) = cache.get(token) {
            // Check if token is still valid
            if info.expires_at > Utc::now() && info.is_valid {
                return Some(TokenInfo {
                    user_id: info.user_id.clone(),
                    expires_at: info.expires_at,
                    is_valid: info.is_valid,
                });
            }
        }
        None
    }

    // Invalidate a token (for logout)
    pub fn invalidate_token(&self, token: &str) {
        let mut cache = self.cache.write().unwrap();
        if let Some(info) = cache.get_mut(token) {
            info.is_valid = false;
        }
    }

    // Clean expired tokens (could be called periodically)
    #[allow(dead_code)]
    pub fn clean_expired(&self) {
        let mut cache = self.cache.write().unwrap();
        let now = Utc::now();
        cache.retain(|_, info| info.expires_at > now && info.is_valid);
    }
}