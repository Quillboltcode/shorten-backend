pub fn convert_axum_to_reqwest_headers(axum_headers: &axum::http::HeaderMap) -> reqwest::header::HeaderMap {
    let mut reqwest_headers = reqwest::header::HeaderMap::new();
    
    // Copy over the authorization header if it exists
    if let Some(auth_header) = axum_headers.get("Authorization") {
        reqwest_headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(auth_header.to_str().unwrap_or_default())
                .expect("Failed to parse auth header"),
        );
    }
    
    reqwest_headers
}