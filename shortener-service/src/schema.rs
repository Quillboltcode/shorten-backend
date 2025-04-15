diesel::table! {
    url_mapping (short_url) {
        short_url -> Varchar,
        alias -> Nullable<Varchar>,
        long_url -> Varchar,
        creation_date -> Timestamp,
        expiration_date -> Nullable<Timestamp>,
        user_id -> Nullable<Int4>,  
        click_count -> Int4,
    }
}

