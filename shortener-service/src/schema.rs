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

diesel::table! {
    account (user_id) {
        user_id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        create_at -> Timestamp,
    }
}

diesel::joinable!(url_mapping -> account (user_id));
diesel::allow_tables_to_appear_in_same_query!(url_mapping, account);
