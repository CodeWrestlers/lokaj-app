use super::schema::messages;
use chrono::prelude::*;

#[derive(Queryable)]
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
