// handlers/mod.rs
pub mod health;
pub mod shortener;
pub mod redirect;
pub mod user;
pub mod auth;

// Re-export all handlers
pub use health::health;
pub use shortener::shorten_url;
pub use redirect::redirect_url;
pub use user::{get_all_users,register_user, get_user, update_user, delete_user, change_password};
pub use auth::{login, logout, refresh_token, validate_token};