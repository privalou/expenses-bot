use futures::StreamExt;
use log::{error, info};
use telegram_bot::{Api, MessageKind, UpdateKind};

use crate::bot::commands::{feedback, help, send_now, start, stop};
use crate::bot::dialogs::{Dialog, Feedback, Start};
use crate::bot::error::BotError;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

const ERROR_TEXT: &str = r#"
Looks like I'm having a technical glitch. Something went wrong.
If the issues persist send feedback via /feedback command.
"#;

pub async fn init_bot(token: &str, author_id: &str) {
    let api = Api::new(&token);
    let telegram_client = TelegramClient::new(token.to_string());

    let handle_message_closure =
        |data: String, user_id: String| handle_message(&telegram_client, author_id, data, user_id);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        if let Ok(update) = update {
            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text { data, .. } = message.kind {
                    let user_id = message.from.id.to_string();
                    if let Err(e) = handle_message_closure(data, user_id.clone()).await {
                        error!("error handling message: {}", e);
                        telegram_client
                            .send_message(&Message {
                                chat_id: &user_id,
                                text: ERROR_TEXT,
                                ..Default::default()
                            })
                            .await
                            .ok();
                    }
                }
            }
        }
    }
}

pub async fn handle_message(
    telegram_client: &TelegramClient,
    author_id: &str,
    payload: String,
    user_id: String,
) -> Result<(), BotError> {
    info!("received message from: {}, message: {}", user_id, payload);

    // TODO: Extract commands as enum
    match payload.as_ref() {
        "/start" => start(&telegram_client, &user_id).await?,
        "/stop" => stop(&telegram_client, &user_id).await?,
        "/feedback" => feedback(&telegram_client, &author_id, &user_id).await?,
        "/sendnow" => send_now(&telegram_client, &user_id).await?,
        "/help" => help(&telegram_client, &user_id).await?,
        _ => {
            //TODO: WTF REMOVE THIS SHIT
            if payload.starts_with("/feedback") {
                let mut dialog = Dialog::<Feedback>::new_with(user_id.clone(), Feedback::Input);
                dialog
                    .handle_current_step(&telegram_client, &user_id, &payload)
                    .await?;
                if payload == "₽" || payload == "€" || payload == "$" {
                    let mut dialog = Dialog::<Start>::new_with(user_id.clone(), Start::Currency);
                    dialog
                        .handle_current_step(&telegram_client, &user_id, &payload)
                        .await?;
                } else {
                    telegram_client
                        .send_message(&Message {
                            chat_id: &user_id,
                            text: "I didn't get that. Use /help to see list of available commands.",
                            ..Default::default()
                        })
                        .await?;
                }
            }
        }
    }
    Ok(())
}
