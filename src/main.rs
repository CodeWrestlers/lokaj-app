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
use teloxide::types::{MediaKind, MessageKind, User};
use teloxide::utils::command::BotCommand;
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

    let bot = Bot::from_env().auto_send();
    let dp = Dispatcher::new(bot).messages_handler(handler);

    log::info!("Starting Lokaj Bot");
    dp.dispatch().await;
}

async fn handler(rx: DispatcherHandlerRx<AutoSend<Bot>, teloxide::prelude::Message>) {
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

            log::trace!("Handling a command");
            match &message.update.kind {
                MessageKind::Common(msg_data) => match &msg_data.media_kind {
                    MediaKind::Text(t) => {
                        log::trace!("Looking for commands in text message...");
                        let command = Command::parse(&t.text, "");
                        let answer = command_handler(command.unwrap(), &user_id).await;
                        message.answer(answer).await;
                        log::trace!("...finished looking for commands");
                    }
                    x => {
                        log::debug!("MediaKind handling not implemented. {:#?}", x);
                    }
                },
                x => {
                    log::debug!("MessageKind handling not implemented. {:#?}", x);
                }
            }

            message
                .answer_dice()
                .await
                .expect("Error while replying with dice");
        })
        .await;
}

#[derive(BotCommand)]
#[command(
    rename = "lowercase",
    description = "Bot przyjmuje pierwszą wpisaną komendę. Dostępne komendy:"
)]
enum Command {
    #[command(description = "wyświetla ten komunikat.")]
    Help,
    #[command(description = "dodaje do listy subskrybentów")]
    Subscribe,
    #[command(description = "usuwa z listy subskrybentów")]
    Unsubscribe,
}

// TODO: make it async
async fn command_handler(cmd: Command, uid: &i64) -> String {
    match cmd {
        Command::Help => Command::descriptions(),
        Command::Subscribe => subscribe(uid),
        Command::Unsubscribe => unsubscribe(uid),
    }
}

fn subscribe(uid: &i64) -> String {
    let answer = format!("Subscribed user_id = {}", uid);
    subscription_update(uid, true);
    answer
}

fn unsubscribe(uid: &i64) -> String {
    let answer = format!("Unsubscribed user_id = {}", uid);
    subscription_update(uid, false);
    answer
}

fn subscription_update(uid: &i64, should_subscribe: bool) {
    let user = get_user(uid);
    match user {
        Some(u) => {
            update_user_subscription(&u, should_subscribe);
        }
        None => {
            log::error!("No user in database!");
        }
    }
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

fn update_user_subscription(u: &models::User, sub: bool) {
    use schema::users::dsl::*;

    let conn = DB
        .get_connection()
        .expect("Error retrieving connection from the pool");

    let result = diesel::update(u)
        .set(is_subscribed.eq(sub))
        .get_result::<models::User>(&conn)
        .expect("Error updating subscription for user");

    log::info!("User {} subscribed!", u.id);
}
