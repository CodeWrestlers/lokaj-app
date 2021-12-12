use crate::database::models::{NewUser, User};
use crate::database::schema::users;
use crate::database::schema::users::dsl::*;
use crate::database::DB;
use chrono::Utc;
use diesel::prelude::*;
use teloxide::types::User as TeloxideUser;

pub async fn save(u: &TeloxideUser) {
    let user_check = get(&u.id);
    match user_check {
        // TODO: update user info
        Some(_) => log::trace!("User exists, no need to save."),
        None => {
            let timestamp = Utc::now();
            let new_user = NewUser {
                id: &u.id,
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
                .get_result::<User>(&conn)
                .expect("Error saving user");

            log::trace!("User saved!");
        }
    }
}

pub fn get(uid: &i64) -> Option<User> {
    let conn = DB
        .get_connection()
        .expect("Error retrieving connection from the pool");

    let result = users
        .filter(id.eq(uid))
        .limit(1)
        .load::<User>(&conn)
        .expect("Error loading user");

    log::debug!("Loaded user: {:#?}", result);

    if result.len() == 1 {
        result.first().cloned()
    } else {
        None
    }
}

pub fn update_subscription(u: &User, sub: bool) {
    let conn = DB
        .get_connection()
        .expect("Error retrieving connection from the pool");

    diesel::update(u)
        .set(is_subscribed.eq(sub))
        .get_result::<User>(&conn)
        .expect("Error updating subscription for user");

    log::info!("User {} subscribed!", u.id);
}
