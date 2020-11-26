use futures::StreamExt;
use log::{error, info};
use telegram_bot::{Api, MessageKind, UpdateKind};

use crate::bot::commands::{feedback, help, start, stop};
use crate::bot::dialogs::{Dialog, Feedback, Start};
use crate::bot::error::BotError;
use crate::store::simple_store::AppStore;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

const ERROR_TEXT: &str = r#"
Looks like I'm having a technical glitch. Something went wrong.
If the issues persist send feedback via /feedback command.
"#;

pub async fn init_bot(token: &str) {
    let api = Api::new(&token);
    let mut store = AppStore::new();
    let telegram_client = TelegramClient::new(token.to_string());

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        if let Ok(update) = update {
            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text { data, .. } = message.kind {
                    let user_id = message.from.id.to_string();
                    if let Err(e) =
                        handle_message(&mut store, &telegram_client, data, &user_id).await
                    {
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
    store: &mut AppStore,
    telegram_client: &TelegramClient,
    payload: String,
    user_id: &str,
) -> Result<(), BotError> {
    info!("received message from: {}, message: {}", user_id, payload);

    // TODO: Extract commands as enum
    match payload.as_ref() {
        "/start" => start(store, &telegram_client, &user_id).await?,
        "/stop" => stop(&telegram_client, &user_id).await?,
        "/feedback" => feedback(store, &telegram_client, &user_id).await?,
        "/help" => help(&telegram_client, &user_id).await?,
        _ => handle_not_a_command_message(store, &telegram_client, &user_id, &payload).await?,
    }
    Ok(())
}

/// process if this message received from registered user else send don't get message
async fn handle_not_a_command_message(
    store: &mut AppStore,
    telegram_client: &TelegramClient,
    user_id: &str,
    payload: &str,
) -> Result<(), BotError> {
    if let Some(dialog_from_store) = store.get_user_dialog(user_id) {
        match dialog_from_store.command.as_str() {
            "/start" => {
                let mut dialog: Dialog<Start> = Dialog::from(dialog_from_store);
                dialog
                    .handle_current_step(store, telegram_client, user_id, payload)
                    .await?
            }
            "/feedback" => {
                let mut dialog: Dialog<Feedback> = Dialog::from(dialog_from_store);
                dialog
                    .handle_current_step(store, telegram_client, user_id, payload)
                    .await?
            }
            _ => {}
        }
        Ok(())
    } else {
        telegram_client
            .send_message(&Message {
                chat_id: &user_id,
                text: "I didn't get that. Use /help to see list of available commands.",
                ..Default::default()
            })
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use mockito::server_url;

    use crate::store::simple_store::DialogPatch;
    use crate::telegram::test_helpers::mock_send_message_success;
    use crate::telegram::types::{
        InlineKeyboardButton, InlineKeyboardMarkup, Message, ReplyMarkup,
    };

    use super::*;

    const TOKEN: &str = "token";
    const USER_ID: &str = "123";

    /// This is a test for parsing /start command from a user who was not in a store before this
    /// message.
    /// As a result handle message should not throw an error and the store should have a new record
    /// about user and his dialogs.
    #[tokio::test]
    async fn handle_message_start_first_step() {
        //given
        let mut store = AppStore::new();
        store.save_user(&USER_ID);

        let url = &server_url();

        let inline_keyboard = vec![vec![
            InlineKeyboardButton {
                text: "₽".to_string(),
                callback_data: "₽".to_string(),
            },
            InlineKeyboardButton {
                text: "$".to_string(),
                callback_data: "$".to_string(),
            },
            InlineKeyboardButton {
                text: "€".to_string(),
                callback_data: "€".to_string(),
            },
        ]];
        let markup = ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup { inline_keyboard });
        let start_first_step_success_action = Message {
            chat_id: &USER_ID,
            text: "Choose your currency",
            reply_markup: Some(&markup),
            ..Default::default()
        };

        let _mock = mock_send_message_success(TOKEN, &start_first_step_success_action);

        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));
        let message =
            handle_message(&mut store, &telegram_client, "/start".to_string(), USER_ID).await;

        //expect
        assert_eq!((), message.expect("Can not handle message"));
        let dialog = store
            .get_user_dialog(&USER_ID)
            .expect("There is no user with such ID");
        assert_eq!("/start", dialog.command);
        assert_eq!("{}".to_string(), dialog.data);
        assert_eq!("Currency".to_string(), dialog.step);
    }

    #[tokio::test]
    async fn handle_message_currency_step() {
        //given
        let mut store = AppStore::new();
        let patch = DialogPatch::new_with(
            Some("/start".to_string()),
            Some("Currency".to_string()),
            None,
        );
        store.save_user(&USER_ID);
        store.update_dialog(patch, USER_ID);

        let url = &server_url();
        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));

        let start_end_step_message = Message {
            chat_id: USER_ID,
            text: "Your currency is €",
            ..Default::default()
        };
        let _mock = mock_send_message_success(TOKEN, &start_end_step_message);

        let message = handle_message(&mut store, &telegram_client, "€".to_string(), USER_ID).await;

        //expect
        assert_eq!((), message.expect("Can not handle message"))
    }
}
