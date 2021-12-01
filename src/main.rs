pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{Message, NewMessage};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use dotenv::dotenv;
use std::env;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();

    log::info!("Starting Lokaj Bot");
    let bot = Bot::from_env().auto_send();

    let pg_connection_manager = r2d2::ConnectionManager::new(get_connection_string());
    let pg_connection_pool = r2d2::Pool::builder()
        .max_size(4)
        .build(pg_connection_manager)
        .unwrap();

    teloxide::repl(bot, |message| async move {
        log::debug!("{:#?}", message.update.from());
        log::debug!("{:#?}", message.update.text());

        let user_id = message.update.from().unwrap().id;
        let text = message.update.text().unwrap();
        log::info!("Connecting to PostgreSQL");
        let connection = pool.get().unwrap();
        receive_message(&connection, &user_id, &text);

        message.answer_dice().await?;
        respond(())
    })
    .await;
}

fn get_connection_string() -> String {
    dotenv().ok();

    let database_url_env = env::var("DATABASE_URL");

    match database_url_env {
        Ok(url) => url,
        Err(_) => {
            let database_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
            let database_pass = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");
            let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
            format!(
                "postgres://{}:{}@postgres/{}",
                database_user, database_pass, database_name
            )
        }
    }
}

fn receive_message<'a>(conn: &PgConnection, user_id: &'a i64, text: &'a str) -> Message {
    use schema::messages;

    let new_message = NewMessage {
        user_id: user_id,
        text: text,
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result(conn)
        .expect("Error saving received message")
}
