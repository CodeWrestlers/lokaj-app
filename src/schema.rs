table! {
    messages (id) {
        id -> Int8,
        user_id -> Int8,
        text -> Text,
        utc_timestamp -> Timestamptz,
        unix_timestamp -> Int4,
    }
}

table! {
    users (user_id) {
        user_id -> Int8,
        is_bot -> Bool,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        language_code -> Nullable<Varchar>,
        is_subscribed -> Bool,
        utc_created -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
