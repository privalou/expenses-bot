use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::bot::dialogs::Dialog;
use crate::bot::error::BotError;
use crate::store::simple_store::{DialogEntity, Store, UserDataPatch};
use crate::telegram::client::TelegramClient;
use crate::telegram::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, ReplyMarkup};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
pub enum Start {
    UnknownRegistrationStatus,
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
            current_step: Some(Start::UnknownRegistrationStatus),
        }
    }

    pub fn new_with(current_step: Start) -> Self {
        Dialog {
            command: "/start".to_string(),
            current_step: Some(current_step),
        }
    }

    pub async fn handle_current_step(
        &mut self,
        store: &mut Store,
        telegram_client: &TelegramClient,
        user_id: &str,
        payload: &str,
    ) -> Result<String, BotError> {
        info!("Received {} payload from user {}", payload, user_id);

        let step = match self.current_step {
            None => Start::UnknownRegistrationStatus,
            Some(value) => value,
        };

        let result: Result<String, BotError> = match step {
            Start::CurrencySelection => {
                self.current_step = Some(Start::AlreadyRegistered);
                info!("received payload at Currency step {}", &payload);
                let dialog_entity =
                    DialogEntity::new_with("/start".to_string(), Some(step.to_string()));
                match dialog_entity {
                    Ok(_) => {
                        store
                            .update_user_data(
                                UserDataPatch::new_with(
                                    Some(
                                        dialog_entity
                                            .expect("Nu nado podumat pochemu tut ne None."),
                                    ),
                                    Some(payload.to_string()),
                                    None,
                                ),
                                &user_id,
                            )
                            .expect("No such user at store");
                        let text_sent_to_user = telegram_client
                            .send_message(&Message {
                                chat_id: user_id,
                                text: format!("Your currency is {}", payload).as_str(),
                                ..Default::default()
                            })
                            .await?;
                        Ok(text_sent_to_user)
                    }
                    Err(_) => Err(BotError::AnotherError(
                        "Could not sent message because of invalid".to_string(),
                    )),
                }
            }
            Start::AlreadyRegistered => {
                info!(
                    "received payload at AlreadyRegistered step from user {}, {}",
                    &payload, &user_id
                );
                let sent_to_user_text_message = telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: "You are already registered. Use /help to see list of available commands.",
                        ..Default::default()
                    }).await?;
                Ok(sent_to_user_text_message)
            }
            Start::UnknownRegistrationStatus => {
                if store.is_registered(user_id) {
                    self.current_step = Some(Start::AlreadyRegistered);
                    Ok(telegram_client.send_message(&Message {
                        chat_id: user_id,
                        text: "You are already registered. Use /help to see list of available commands.",
                        ..Default::default()
                    }).await?)
                } else {
                    self.current_step = Some(Start::CurrencySelection);
                    store.save_user(&user_id);
                    let result_message_text = telegram_client
                        .send_message(&Message {
                            chat_id: user_id,
                            text: "Choose your currency",
                            reply_markup: Some(&ReplyMarkup::InlineKeyboardMarkup(
                                InlineKeyboardMarkup {
                                    inline_keyboard: vec![vec![
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
                                    ]],
                                },
                            )),
                            ..Default::default()
                        })
                        .await
                        .expect("hz chto ne tak");
                    Ok(result_message_text)
                }
            }
        };
        Ok(result.expect("HZ CHTO ZA HUETA"))
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
        let mut store = Store::new();

        let url = &server_url();

        let markup = ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
            inline_keyboard: vec![vec![
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
            ]],
        });
        let first_step_default_message = Message {
            chat_id: USER_ID,
            text: "Choose your currency",
            reply_markup: Some(&markup),
            ..Default::default()
        };

        let mock = mock_send_message_success(TOKEN, &first_step_default_message);

        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));

        let mut dialog = Dialog::<Start>::new();
        let received_text = dialog
            .handle_current_step(&mut store, &telegram_client, USER_ID, "")
            .await
            .expect("Can not process start step");
        assert_eq!(received_text, first_step_default_message.text);

        mock.assert();
    }
    //
    // #[tokio::test]
    // async fn handle_current_step_response_for_registered_user() {
    //     let mut store = AppStore::new();
    //     store.save_user(USER_ID);
    //
    //     let url = &server_url();
    //
    //     let first_step_response_to_registered_user = Message {
    //         chat_id: USER_ID,
    //         text: "You are already registered. ",
    //         ..Default::default()
    //     };
    // }

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

        let mut dialog = Dialog::<Start>::new_with(Start::CurrencySelection);
        let received_text_message = dialog
            .handle_current_step(&mut store, &telegram_client, USER_ID, "$")
            .await
            .expect("Can not process start step");
        assert_eq!(received_text_message, start_message_current_step.text);

        mock.assert();
    }
}
