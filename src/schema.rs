table! {
    messages (id) {
        id -> Int8,
        user_id -> Int8,
        text -> Text,
    }
}

table! {
    users (id) {
        id -> Int8,
        is_bot -> Bool,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        language_code -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
