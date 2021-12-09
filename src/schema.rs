table! {
    garbage_collection (id) {
        id -> Int4,
        garbage_type_id -> Int4,
        collection_date -> Date,
    }
}

table! {
    garbage_types (id) {
        id -> Int4,
        name -> Varchar,
        language_code -> Varchar,
    }
}

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
    users (id) {
        id -> Int8,
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
    garbage_collection,
    garbage_types,
    messages,
    users,
);
