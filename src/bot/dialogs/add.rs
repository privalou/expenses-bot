use std::str::FromStr;

use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::bot::dialogs::Dialog;
use crate::bot::error::BotError;
use crate::store::history::ExpenseRecordPatch;
use crate::store::Store;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
pub enum Add {
    Amount,
    Category,
}

/// Right now this step is a default state after registration.
///
impl Default for Dialog<Add> {
    fn default() -> Self {
        Self::new()
    }
}

impl Dialog<Add> {
    pub fn new() -> Self {
        Dialog {
            command: "/add".to_string(),
            current_step: None,
        }
    }

    pub fn new_with(current_step: Add) -> Self {
        Dialog {
            command: "/add".to_string(),
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
            Some(Add::Amount) => match f32::from_str(payload) {
                Ok(parsed_value_amount) => {
                    match store.add_expense_record(user_id, &parsed_value_amount) {
                        Ok(_) => Ok(telegram_client
                            .send_message(&Message {
                                chat_id: &user_id,
                                text: format!(
                                    "Write a category where you have spent {}.",
                                    parsed_value_amount
                                )
                                .as_str(),
                                ..Default::default()
                            })
                            .await?),
                        Err(err) => Err(BotError::StoreError(err)),
                    }
                }
                Err(err) => Err(BotError::ParsingError(err)),
            },
            Some(Add::Category) => {
                match store.update_latest_record(
                    user_id,
                    ExpenseRecordPatch::new(Some(payload.to_string())),
                ) {
                    Ok(_) => Ok(telegram_client
                        .send_message(&Message {
                            chat_id: &user_id,
                            text: "Record has been saved",
                            ..Default::default()
                        })
                        .await?),
                    Err(err) => Err(BotError::StoreError(err)),
                }
            }
            None => Ok(telegram_client
                .send_message(&Message {
                    chat_id: &user_id,
                    text: "Write amount of money you have spent",
                    ..Default::default()
                })
                .await?),
        }
    }
}

#[cfg(test)]
mod tests {
    use mockito::server_url;

    use crate::telegram::test_helpers::mock_send_message_success;

    use super::*;

    const TOKEN: &str = "token";
    const USER_ID: &str = "123";

    #[tokio::test]
    async fn handle_current_step_none() {
        let store = Store::new();

        let sent_message_current_step_none = Message {
            chat_id: USER_ID,
            text: "Write amount of money you have spent",
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &sent_message_current_step_none);
        let telegram_client =
            TelegramClient::new_with(String::from(TOKEN), String::from(&server_url()));

        let dialog: Dialog<Add> = Dialog::<Add>::new();
        let received_text = dialog
            .handle_current_step(&store, &telegram_client, USER_ID, "")
            .await
            .expect("Can not process add step");

        assert_eq!(received_text, sent_message_current_step_none.text);

        mock.assert();
    }

    #[tokio::test]
    async fn handle_amount_current_step_valid_payload_with_user_at_store() {
        let store = Store::new();
        store.save_user(USER_ID);

        let sent_message_amount_current_step = Message {
            chat_id: USER_ID,
            text: "Write a category where you have spent 30.",
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &sent_message_amount_current_step);
        let telegram_client =
            TelegramClient::new_with(String::from(TOKEN), String::from(&server_url()));

        let dialog: Dialog<Add> = Dialog::<Add>::new_with(Add::Amount);
        let received_text = dialog
            .handle_current_step(&store, &telegram_client, USER_ID, "30")
            .await
            .expect("Can not process add step");

        assert_eq!(received_text, sent_message_amount_current_step.text);

        mock.assert();
    }
}
