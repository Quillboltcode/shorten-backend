diesel::table! {
    short_urls (id) {
        id -> Uuid,
        short_code -> Varchar,
        original_url -> Text,
        created_at -> Timestamp,
    }
}
