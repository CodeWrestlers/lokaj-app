use crate::database::models::{GarbageCollection, GarbageTypes};
use crate::database::schema::garbage_collection::dsl::*;
use crate::database::schema::garbage_types::dsl::*;
use crate::database::DB;
use chrono::prelude::*;
use chrono::Duration;
use diesel::prelude::*;

pub fn get() -> Vec<(GarbageCollection, GarbageTypes)> {
    let conn = DB
        .get_connection()
        .expect("Error retrieving connection from the pool");

    let today = Utc::now().naive_utc().date();
    let dur = Duration::weeks(4);
    let weeks_from_today = today + dur;

    let timetable: Vec<(GarbageCollection, GarbageTypes)> = garbage_collection
        .filter(collection_date.between(&today, &weeks_from_today))
        .inner_join(garbage_types)
        .order(collection_date.asc())
        .load(&conn)
        .expect("Error loading garbage timetable");

    timetable
}
