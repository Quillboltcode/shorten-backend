use crate::models::user::{NewUser, UserModel};
use axum::{extract::State, routing::post, Json, Router};
use bcrypt::{hash, verify, DEFAULT_COST};
use common::db::DbPool;
use diesel::{prelude::*, r2d2::Pool};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use uuid::Uuid;
#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

/// JWT Claims
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// Registers a new user
async fn register(
    State(state): State<Arc<DbPool>>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<UserModel>, String> {
    let conn = &mut state.get().map_err(|_| "DB Connection Failed")?;

    // Hash password
    let hashed_password =
        hash(&payload.password, DEFAULT_COST).map_err(|_| "Password hashing failed")?;

    let new_user = NewUser {
        email: payload.email.clone(),
        password_hash: hashed_password,
    };

    diesel::insert_into(crate::schema::account::table)
        .values(&new_user)
        .execute(conn)
        .map_err(|_| "Failed to create user")?;

    // Retrieve and return the newly created user (without password)
    let user = crate::schema::account::table
        .select((
            crate::schema::account::user_id,
            crate::schema::account::email,
            crate::schema::account::password_hash,
            crate::schema::account::create_at,
        ))
        .filter(crate::schema::account::email.eq(&payload.email))
        .first::<UserModel>(conn)
        .map_err(|_| "User retrieval failed")?;

    Ok(Json(user))
}

/// Logs in a user and returns a JWT token
async fn login(
    State(state): State<Arc<DbPool>>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, String> {
    let conn = &mut state.get().map_err(|_| "DB Connection Failed")?;

    let user = crate::schema::account::table
    .select((crate::schema::account::user_id, crate::schema::account::email))
    .filter(crate::schema::account::email.eq(&payload.email))
    .first::<(i32, String)>(conn)
    .map_err(|_| "User retrieval failed")?;

    let (user_id, hashed_password) = user;

    if !verify(&payload.password, &hashed_password).map_err(|_| "Password verification failed")? {
        return Err("Invalid credentials".to_string());
    }

    // Generate JWT
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| "Token generation failed")?;

    Ok(Json(AuthResponse { token }))
}

/// Authentication routes
pub fn auth_routes() -> Router<Arc<DbPool>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
