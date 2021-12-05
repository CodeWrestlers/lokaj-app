pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate lazy_static;

use self::models::{Message, NewMessage};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use teloxide::dispatching::Dispatcher;
use teloxide::prelude::*;
use tokio_stream::wrappers::UnboundedReceiverStream;

type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

struct Db {
    pool: PgPool,
}

impl Db {
    fn get_connection(&self) -> Result<PgPooledConnection, PoolError> {
        self.pool.get()
    }
}

// TODO: pool size parameter as environment var
lazy_static! {
    static ref DB: Db = {
        let cm = ConnectionManager::<PgConnection>::new(get_connection_string());
        Db {
            pool: Pool::builder().max_size(4).build(cm).unwrap(),
        }
    };
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();

    log::info!("Starting Lokaj Bot");
    let bot = Bot::from_env().auto_send();

    let dp = Dispatcher::new(bot).messages_handler(
        |rx: DispatcherHandlerRx<AutoSend<Bot>, teloxide::prelude::Message>| async move {
            UnboundedReceiverStream::new(rx)
                .for_each_concurrent(None, |message| async move {
                    log::info!("Received a message!");
                    let user_id = message.update.from().unwrap().id;
                    let text = message.update.text().unwrap();

                    log::info!("Saving message to database...");
                    receive_message(&user_id, &text).await;

                    message
                        .answer_dice()
                        .await
                        .expect("Error while replying with dice");
                })
                .await;
        },
    );

    dp.dispatch().await;
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

async fn receive_message<'a>(user_id: &'a i64, text: &'a str) {
    use schema::messages;

    let new_message = NewMessage {
        user_id: user_id,
        text: text,
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
