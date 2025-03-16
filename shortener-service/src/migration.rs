use diesel::prelude::*;
use crate::models::user::{NewUser, UserModel};
use crate::models::url::UrlMappingModel; 
use chrono::Utc;
use diesel::select;
use diesel::dsl::exists;
pub fn seed_data(conn: &mut PgConnection) {
    use crate::schema::{account::dsl::*, url_mapping::dsl::*};
    
    // Check if users already exist
    let user_exists: bool = select(exists(account.limit(1)))
    .get_result(conn)
    .unwrap_or(false);

    if !user_exists {
        let users = vec![
            NewUser {
                email: "alice@example.com".to_string(),
                password_hash: "hashedpassword1".to_string(),
            },
            NewUser {
                email: "bob@example.com".to_string(),
                password_hash: "hashedpassword2".to_string(),
            },
        ];

        diesel::insert_into(account).values(&users).execute(conn).expect("Failed to insert users");
    }

    // Check if URLs already exist
    let url_exists: bool = diesel::select(diesel::dsl::exists(url_mapping.select(short_url)))
        .get_result(conn)
        .unwrap_or(false);

    if !url_exists {
        let urls = vec![
            UrlMappingModel {
                short_url: "abc123".to_string(),
                alias: Some("example".to_string()),
                long_url: "https://example.com".to_string(),
                creation_date: Utc::now().naive_utc(),
                expiration_date: None,
                user_id: Some(1),
                click_count: 0,
            },
            UrlMappingModel {
                short_url: "xyz789".to_string(),
                alias: Some("rust".to_string()),
                long_url: "https://rust-lang.org".to_string(),
                creation_date: Utc::now().naive_utc(),
                expiration_date: None,
                user_id: Some(2),
                click_count: 0,
            },
            UrlMappingModel {
                short_url: "free456".to_string(),
                alias: None,
                long_url: "https://opensource.org".to_string(),
                creation_date: Utc::now().naive_utc(),
                expiration_date: None,
                user_id: None,
                click_count: 0,
            },
        ];

        diesel::insert_into(url_mapping).values(&urls).execute(conn).expect("Failed to insert URL mappings");
    }
}
