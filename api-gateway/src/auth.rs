use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;

use crate::app_state::AppState;
use crate::models::Claims;

pub async fn validate_token(headers: &HeaderMap, state: &Arc<AppState>, token_type: &str) -> Result<Claims, String> {
    // Extract token from Authorization header
    let auth_header = headers
        .get("Authorization")
        .ok_or("No authorization header")?
        .to_str()
        .map_err(|_| "Invalid authorization header")?;
    
    // Check if it starts with "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err("Invalid authorization format".to_string());
    }
    
    // Extract the token
    let token = &auth_header[7..]; // Skip "Bearer "
    
    // Check if token is in cache
    if let Some(token_info) = state.token_cache.get_token(token) {
        // Token found in cache and is valid
        return Ok(Claims {
            sub: token_info.user_id.clone(),
            exp: token_info.expires_at.timestamp() as usize,
            token_type: token_type.to_string(),
            user_id: Some(token_info.user_id),
        });
    }
    
    // Token not in cache, validate with user service
    match state
        .user_service_client
        .post(format!("{}/auth/validate-token", state.user_service_url))
        .json(&serde_json::json!({ "token": token, "token_type": token_type }))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                // Parse response
                let validation_result: serde_json::Value = response.json().await
                    .map_err(|_| "Failed to parse validation response".to_string())?;
                
                // Check if token is valid
                let is_valid = validation_result["valid"]
                    .as_bool()
                    .unwrap_or(false);
                
                if !is_valid {
                    return Err("Invalid token".to_string());
                }
                
                // Extract token data
                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
                    &Validation::default(),
                )
                .map_err(|_| "Failed to decode token".to_string())?;
                
                // Check token type
                if token_data.claims.token_type != token_type {
                    return Err("Invalid token type".to_string());
                }
                
                // Add to cache for future use
                if let Some(user_id) = &token_data.claims.user_id {
                    let expires_at = Utc::now() + Duration::seconds((token_data.claims.exp as i64) - Utc::now().timestamp());
                    state.token_cache.store_token(token, user_id.clone(), expires_at);
                }
                
                Ok(token_data.claims)
            } else {
                Err("Invalid token".to_string())
            }
        }
        Err(_) => Err("Failed to validate token with user service".to_string()),
    }
}