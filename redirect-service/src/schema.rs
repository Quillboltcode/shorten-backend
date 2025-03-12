diesel::table! {
    short_urls (id) {
        id -> Int4,
        short_code -> Varchar,
        original_url -> Text,
    }
}
