use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::bot::dialogs::Dialog;
use crate::bot::error::BotError;
use crate::store::{DialogEntity, Store, UserDataPatch};
use crate::telegram::client::TelegramClient;
use crate::telegram::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, ReplyMarkup};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
pub enum Start {
    CurrencySelection,
    AlreadyRegistered,
}

impl Default for Dialog<Start> {
    fn default() -> Self {
        Self::new()
    }
}

impl Dialog<Start> {
    pub fn new() -> Self {
        Dialog {
            command: "/start".to_string(),
            current_step: None,
        }
    }

    pub fn new_with(current_step: Start) -> Self {
        Dialog {
            command: "/start".to_string(),
            current_step: Some(current_step),
        }
    }

    pub async fn handle_current_step(
        &self,
        store: &Store,
        telegram_client: &TelegramClient,
        user_id: &str,
        payload: &str,
    ) -> Result<String, BotError> {
        info!("Received {} payload from user {}", payload, user_id);

        match self.current_step {
            Some(Start::CurrencySelection) => {
                info!("received payload at Currency step {}", &payload);
                let dialog_entity = DialogEntity::new_with(
                    "/start".to_string(),
                    Some(Start::AlreadyRegistered.to_string()),
                )
                .expect("Could not sent message because of invalid");
                match store.update_user_data(
                    UserDataPatch::new_with(Some(dialog_entity), Some(payload.to_string())),
                    &user_id,
                ) {
                    Ok(_) => {
                        let text_sent_to_user = telegram_client
                            .send_message(&Message {
                                chat_id: user_id,
                                text: format!("Your currency is {}", payload).as_str(),
                                ..Default::default()
                            })
                            .await?;
                        Ok(text_sent_to_user)
                    }
                    Err(error) => Err(BotError::StoreError(error)),
                }
            }
            Some(Start::AlreadyRegistered) => {
                info!(
                    "received payload at AlreadyRegistered step from user {}, {}",
                    &payload, &user_id
                );
                let text_sent_to_user = telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: "You are already registered. Use /help to see list of available commands.",
                        ..Default::default()
                    }).await?;
                Ok(text_sent_to_user)
            }
            None => {
                if store.is_registered(user_id) {
                    Ok(telegram_client.send_message(&Message {
                        chat_id: user_id,
                        text: "You are already registered. Use /help to see list of available commands.",
                        ..Default::default()
                    }).await?)
                } else {
                    store.save_user(&user_id);
                    let text_sent_to_user = telegram_client
                        .send_message(&Message {
                            chat_id: user_id,
                            text: "Choose your currency",
                            reply_markup: Some(&ReplyMarkup::InlineKeyboardMarkup(
                                InlineKeyboardMarkup {
                                    inline_keyboard: vec![vec![
                                        InlineKeyboardButton::new("₽"),
                                        InlineKeyboardButton::new("$"),
                                        InlineKeyboardButton::new("€"),
                                    ]],
                                },
                            )),
                            ..Default::default()
                        })
                        .await?;
                    Ok(text_sent_to_user)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use mockito::server_url;

    use crate::telegram::test_helpers::mock_send_message_success;

    use super::*;

    const TOKEN: &str = "token";
    const USER_ID: &str = "123";

    #[tokio::test]
    async fn handle_current_step_success_start_first_step_with_not_registered_user() {
        let store = Store::new();

        let markup = ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
            inline_keyboard: vec![vec![
                InlineKeyboardButton::new("₽"),
                InlineKeyboardButton::new("$"),
                InlineKeyboardButton::new("€"),
            ]],
        });
        let first_step_default_message = Message {
            chat_id: USER_ID,
            text: "Choose your currency",
            reply_markup: Some(&markup),
            ..Default::default()
        };

        let mock = mock_send_message_success(TOKEN, &first_step_default_message);

        let telegram_client =
            TelegramClient::new_with(String::from(TOKEN), String::from(&server_url()));

        let dialog = Dialog::<Start>::new();
        let received_text = dialog
            .handle_current_step(&store, &telegram_client, USER_ID, "")
            .await
            .expect("Can not process start step");
        assert_eq!(received_text, first_step_default_message.text);

        mock.assert();
    }

    #[tokio::test]
    async fn handle_current_step_response_for_registered_user() {
        let mut store = Store::new();
        store.save_user(USER_ID);

        let url = &server_url();

        let unknown_registration_status_response_to_registered_user = Message {
            chat_id: USER_ID,
            text: "You are already registered. Use /help to see list of available commands.",
            ..Default::default()
        };

        let mock = mock_send_message_success(
            TOKEN,
            &unknown_registration_status_response_to_registered_user,
        );

        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));

        let dialog = Dialog::<Start>::new();

        let received_text = dialog
            .handle_current_step(&mut store, &telegram_client, USER_ID, "")
            .await
            .expect("Can not process start step");
        assert_eq!(
            received_text,
            unknown_registration_status_response_to_registered_user.text
        );

        mock.assert();
    }

    #[tokio::test]
    async fn handle_current_step_success_start_currency_step() {
        let mut store = Store::new();
        store.save_user(USER_ID);

        let url = &server_url();

        let start_message_current_step = Message {
            chat_id: USER_ID,
            text: "Your currency is $",
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &start_message_current_step);

        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));

        let dialog = Dialog::<Start>::new_with(Start::CurrencySelection);
        let received_text_message = dialog
            .handle_current_step(&mut store, &telegram_client, USER_ID, "$")
            .await
            .expect("Can not process start step");
        assert_eq!(received_text_message, start_message_current_step.text);

        mock.assert();
    }
}
