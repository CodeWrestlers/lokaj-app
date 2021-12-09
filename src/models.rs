use super::schema::messages;
use super::schema::users;
use chrono::prelude::*;

#[derive(Queryable, Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub text: String,
    pub utc_timestamp: DateTime<Utc>,
    pub unix_timestamp: i32,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub user_id: &'a i64,
    pub text: &'a str,
    pub utc_timestamp: &'a DateTime<Utc>,
    pub unix_timestamp: &'a i32,
}

//
#[derive(Queryable, Debug, Clone, Identifiable)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_subscribed: bool,
    pub utc_created: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub user_id: &'a i64,
    pub is_bot: &'a bool,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub language_code: &'a str,
    pub is_subscribed: &'a bool,
    pub utc_created: &'a DateTime<Utc>,
}
