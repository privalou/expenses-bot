use std::str::FromStr;

use log::{error, info};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::bot::dialogs::{Command, Dialog};
use crate::bot::error::BotError;
use crate::db::models::dialog::DialogEntity;
use crate::db::models::history::HistoryEntity;
use crate::db::Connection;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
pub enum Add {
    Amount,
    Category,
}

impl Default for Dialog<Add> {
    fn default() -> Self {
        Self::new()
    }
}

impl Dialog<Add> {
    pub fn new() -> Self {
        Dialog {
            command: Command::Add,
            current_step: None,
        }
    }

    pub fn new_with(current_step: Add) -> Self {
        Dialog {
            command: Command::Add,
            current_step: Some(current_step),
        }
    }

    pub async fn handle_current_step(
        &self,
        conn: &Connection,
        telegram_client: &TelegramClient,
        user_id: &str,
        payload: &str,
    ) -> Result<String, BotError> {
        info!("Received {} payload from user {}", payload, user_id);

        match self.current_step {
            Some(Add::Amount) => {
                let parsed_value = match f32::from_str(payload) {
                    Ok(value) => value,
                    Err(err) => {
                        telegram_client
                            .send_message(&Message {
                                chat_id: &user_id,
                                text: format!(
                                    "Can not parse: {} to number. Try /add again.",
                                    payload
                                )
                                .as_str(),
                                ..Default::default()
                            })
                            .await?;
                        error!("failed to create new user: {}", err);
                        return Err(BotError::ParsingError(err));
                    }
                };
                HistoryEntity::add_expense_record(user_id.to_string(), parsed_value, conn)?;
                DialogEntity::update_dialog(
                    &DialogEntity::new(
                        user_id.to_string(),
                        Command::Add.to_string(),
                        Some(Add::Category.to_string()),
                    ),
                    conn,
                )?;
                Ok(telegram_client
                    .send_message(&Message {
                        chat_id: &user_id,
                        text: format!("Write a category where you have spent {}.", parsed_value)
                            .as_str(),
                        ..Default::default()
                    })
                    .await?)
            }
            Some(Add::Category) => {
                DialogEntity::update_dialog(
                    &DialogEntity::new(user_id.to_string(), Command::Start.to_string(), None),
                    conn,
                )?;
                Ok(telegram_client
                    .send_message(&Message {
                        chat_id: &user_id,
                        text: "Record has been saved",
                        ..Default::default()
                    })
                    .await?)
            }
            None => {
                DialogEntity::update_dialog(
                    &DialogEntity::new(
                        user_id.to_string(),
                        Command::Add.to_string(),
                        Some(Add::Amount.to_string()),
                    ),
                    conn,
                )?;
                Ok(telegram_client
                    .send_message(&Message {
                        chat_id: &user_id,
                        text: "Write amount of money you have spent",
                        ..Default::default()
                    })
                    .await?)
            }
        }
    }
}
