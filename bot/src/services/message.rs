use crate::database::models::{Message, NewMessage};
use crate::database::schema::messages;
use crate::database::DB;
use chrono::Utc;
use diesel::prelude::*;

pub async fn save<'a>(user_id: &'a i64, text: &'a str, unix_timestamp: &'a i32) {
    let timestamp = Utc::now();
    let new_message = NewMessage {
        user_id,
        text,
        utc_timestamp: &timestamp,
        unix_timestamp,
    };

    let conn = DB
        .get_connection()
        .expect("Error retrieving connection from the pool");

    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result::<Message>(&conn)
        .expect("Error saving received message");

    log::info!("Message saved!");
}
