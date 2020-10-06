use futures::StreamExt;
use log::{error, info, warn};
use telegram_bot::{Api, MessageKind, MessageOrChannelPost, UpdateKind};

use crate::bot::commands::{feedback, help, send_now, start, stop};
use crate::bot::dialogs::{Dialog, Start};
use crate::bot::error::BotError;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

const ERROR_TEXT: &str = r#"
Looks like I'm having a technical glitch. Something went wrong.
If the issues persist send feedback via /feedback command.
"#;

pub async fn init_bot(token: &str, bot_name: &str, author_id: &str) {
    let api = Api::new(&token);
    let telegram_client = TelegramClient::new(token.to_string());

    let handle_message_closure =
        |data: String, user_id: String| handle_message(&telegram_client, author_id, data, user_id);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        if let Ok(update) = update {
            match update.kind {
                UpdateKind::Message(message) => {
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
                UpdateKind::CallbackQuery(query) => {
                    if query.message.is_none() {
                        warn!("empty message in callback query");
                        continue;
                    }

                    if query.data.is_none() {
                        warn!("empty data in callback query");
                        continue;
                    }

                    let message = query.message.unwrap();
                    let data = query.data.unwrap();
                    let user_id;

                    match message {
                        MessageOrChannelPost::Message(message) => {
                            user_id = message.chat.id().to_string();
                        }
                        MessageOrChannelPost::ChannelPost(post) => {
                            user_id = post.chat.id.to_string();
                        }
                    }

                    if let Err(e) = handle_message_closure(data, user_id.clone()).await {
                        error!("error handling message in callback query: {}", e);
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
                UpdateKind::ChannelPost(post) => {
                    if let MessageKind::Text { data, .. } = post.kind {
                        let mut parsed_data = data;
                        // If message ends with bot_name. Replace bot_name with empty string.
                        if parsed_data.ends_with(bot_name) {
                            parsed_data = parsed_data.replace(&format!("@{}", bot_name), "");
                        }

                        let user_id = post.chat.id.to_string();
                        if let Err(e) = handle_message_closure(parsed_data, user_id.clone()).await {
                            error!("error handling channel post: {}", e);
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
                _ => {}
            }
        }
    }
}

async fn handle_message(
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
            if payload == "₽" || payload == "€" || payload == "$" {
                let mut dialog = Dialog::<Start>::currency(user_id.to_string());
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
    Ok(())
}
