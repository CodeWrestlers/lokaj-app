pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{Message, NewMessage};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::mem::size_of;
use std::sync::mpsc::sync_channel;
use std::thread;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();

    log::info!("Starting Lokaj Bot");
    let bot = Bot::from_env().auto_send();

    //let (tx, rx) = sync_channel(size_of::<PgConnection>());

    teloxide::repl(bot, |message| async move {
        log::debug!("{:#?}", message.update.from());
        log::debug!("{:#?}", message.update.text());

        //let user_id = message.update.from().unwrap().id;
        //let text = message.update.text().unwrap();
        //tx.send((&user_id, &text)).unwrap();

        message.answer_dice().await?;
        respond(())
    })
    .await;

    //    thread::spawn(move || {
    //        log::info!("Connecting to PostgreSQL");
    //        let connection = establish_connection();
    //        loop {
    //            let (user_id, text) = rx.recv().unwrap();
    //            receive_message(&connection, &user_id, &text);
    //        }
    //    });
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url_env = env::var("DATABASE_URL");

    let database_url = match database_url_env {
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
    };

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
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
