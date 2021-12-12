pub mod bot;
pub mod commands;
pub mod database;
pub mod services;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate lazy_static;

#[tokio::main]
async fn main() {
    bot::run().await;
}
