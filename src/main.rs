pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate lazy_static;

use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use teloxide::dispatching::Dispatcher;
use teloxide::prelude::*;
use teloxide::types::User;
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
                    log::trace!("Received a message!");
                    let user_id = message.update.from().unwrap().id;
                    let text = message.update.text().unwrap();
                    let unix_timestamp = message.update.date;

                    log::trace!("{:#?}", message.update);

                    log::trace!("Saving message to database...");
                    receive_message(&user_id, &text, &unix_timestamp).await;

                    log::trace!("Saving user to database...");
                    save_user(&message.update.from().unwrap()).await;

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

async fn receive_message<'a>(user_id: &'a i64, text: &'a str, unix_timestamp: &'a i32) {
    use schema::messages;

    let timestamp = Utc::now();
    let new_message = models::NewMessage {
        user_id: user_id,
        text: text,
        utc_timestamp: &timestamp,
        unix_timestamp: unix_timestamp,
    };

    let conn = DB
        .get_connection()
        .expect("Error retrieving connection from the pool");

    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result::<models::Message>(&conn)
        .expect("Error saving received message");

    log::info!("Message saved!");
}

async fn save_user<'a>(u: &'a User) {
    use schema::users;

    let user_check = get_user(&u.id);
    match user_check {
        // TODO: update user info
        Some(u) => log::trace!("User exists, no need to save."),
        None => {
            let timestamp = Utc::now();
            let new_user = models::NewUser {
                user_id: &u.id,
                is_bot: &u.is_bot,
                first_name: &u.first_name,
                last_name: match &u.last_name {
                    Some(x) => x,
                    None => "",
                },
                username: match &u.username {
                    Some(x) => x,
                    None => "",
                },
                language_code: match &u.language_code {
                    Some(x) => x,
                    None => "",
                },
                is_subscribed: &false,
                utc_created: &timestamp,
            };

            let conn = DB
                .get_connection()
                .expect("Error retrieving connection from the pool");

            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result::<models::User>(&conn)
                .expect("Error saving user");

            log::trace!("User saved!");
        }
    }
}

fn get_user(uid: &i64) -> Option<models::User> {
    use schema::users::dsl::*;

    let conn = DB
        .get_connection()
        .expect("Error retrieving connection from the pool");

    let result = users
        .filter(user_id.eq(uid))
        .limit(1)
        .load::<models::User>(&conn)
        .expect("Error loading user");

    log::debug!("Loaded user: {:#?}", result);
    assert_eq!(result.len(), 1);

    result.first().cloned()
}
