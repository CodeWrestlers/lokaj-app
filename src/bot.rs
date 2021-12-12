use crate::commands;
use crate::services;
use std::env;
use teloxide::dispatching::Dispatcher;
use teloxide::prelude::*;
use teloxide::types::{MediaKind, MessageKind};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub async fn run() {
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
            services::message::save(&user_id, text, &unix_timestamp).await;

            log::trace!("Saving user to database...");
            services::user::save(message.update.from().unwrap()).await;

            log::trace!("Handling a command");
            match &message.update.kind {
                MessageKind::Common(msg_data) => match &msg_data.media_kind {
                    MediaKind::Text(t) => {
                        log::trace!("Looking for commands in text message...");
                        let ans = commands::parse(&t.text, &user_id, "").await;
                        match ans {
                            Some(a) => {
                                message
                                    .answer(a)
                                    .await
                                    .expect("Error answering with command result");
                            }
                            None => {}
                        };
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
        })
        .await;
}
