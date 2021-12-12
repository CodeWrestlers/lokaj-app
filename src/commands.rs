use crate::database::models;
use crate::services;
use teloxide::utils::command::BotCommand;

#[derive(BotCommand)]
#[command(
    rename = "lowercase",
    description = "Bot przyjmuje pierwszą wpisaną komendę. Dostępne komendy:"
)]
pub enum Command {
    #[command(description = "wyświetla ten komunikat.")]
    Help,
    #[command(description = "dodaje do listy subskrybentów.")]
    Subscribe,
    #[command(description = "usuwa z listy subskrybentów.")]
    Unsubscribe,
    #[command(description = "listuje planowane wywozy na najbliższe 4 tygodnie.")]
    Timetable,
    #[command(description = "podaje następny wywóz śmieci.")]
    Next,
}

pub async fn parse(text: &str, uid: &i64, name: &str) -> Option<String> {
    let cmd = Command::parse(text, name);
    match cmd {
        Ok(c) => Some(handler(c, uid).await),
        Err(e) => {
            log::debug!("Command match Err: {}", e);
            None
        }
    }
}

// TODO: make it async
async fn handler(cmd: Command, uid: &i64) -> String {
    match cmd {
        Command::Help => Command::descriptions(),
        Command::Subscribe => subscribe(uid),
        Command::Unsubscribe => unsubscribe(uid),
        Command::Timetable => timetable(),
        Command::Next => next_collection(),
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
    let user = services::user::get(uid);
    match user {
        Some(u) => {
            services::user::update_subscription(&u, should_subscribe);
        }
        None => {
            log::error!("No user in database!");
        }
    }
}

fn timetable() -> String {
    let tt = services::timetable::get();

    let tt_vec: Vec<String> = tt
        .iter()
        .map(|(gc, gt)| format!("{} - {} {}", gc.date, gt.emoji, gt.name))
        .collect();

    tt_vec.join("\n")
}

fn next_collection() -> String {
    let tt = services::timetable::get();

    let next_gc: &models::GarbageCollection = match tt.first() {
        Some((gc, _)) => gc,
        None => panic!("Timetable empty!"),
    };
    let next_gc_date = &next_gc.date;

    let tt_vec: Vec<String> = tt
        .iter()
        .filter_map(|(gc, gt)| {
            if &gc.date == next_gc_date {
                Some(format!("{} - {} {}", gc.date, gt.emoji, gt.name))
            } else {
                None
            }
        })
        .collect();

    tt_vec.join("\n")
}
