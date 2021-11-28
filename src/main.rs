use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting Lokaj Bot");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        message.answer_dice().await?;
        respond(())
    })
    .await;
}
