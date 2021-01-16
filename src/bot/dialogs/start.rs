use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::bot::dialogs::{Command, Dialog};
use crate::bot::error::BotError;
use crate::db::models::dialog::DialogEntity;
use crate::db::models::user::UserEntity;
use crate::db::Connection;
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
            command: Command::Start,
            current_step: None,
        }
    }

    pub fn new_with(current_step: Start) -> Self {
        Dialog {
            command: Command::Start,
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

        let current_step = match self.current_step {
            None => match DialogEntity::get_user_dialog(user_id, conn) {
                Err(_) => None,
                Ok(entity) => match entity.step {
                    None => Some(Start::AlreadyRegistered),
                    Some(step) => Some(
                        step.parse()
                            .expect("Can not parse command. Should not happen"),
                    ),
                },
            },
            Some(step) => Some(step),
        };

        match current_step {
            Some(Start::CurrencySelection) => {
                info!("received payload at Currency step {}", &payload);
                let dialog_entity = DialogEntity::new(
                    user_id.to_string(),
                    "/start".to_string(),
                    Some(Start::AlreadyRegistered.to_string()),
                );
                UserEntity::update_currency(payload, user_id, conn)?;
                DialogEntity::update_dialog(&dialog_entity, &conn)?;
                Ok(telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: format!("Your currency is {}", payload).as_str(),
                        ..Default::default()
                    })
                    .await?)
            }
            Some(Start::AlreadyRegistered) => {
                info!(
                    "received payload at AlreadyRegistered step from user {}, {}",
                    &payload, &user_id
                );
                Ok(telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: "You are already registered. Use /help to see list of available commands.",
                        ..Default::default()
                    }).await?)
            }
            None => {
                UserEntity::save_user(user_id, conn)?;
                let reply_markup = ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
                    inline_keyboard: vec![vec![
                        InlineKeyboardButton::new("₽"),
                        InlineKeyboardButton::new("$"),
                        InlineKeyboardButton::new("€"),
                    ]],
                });
                Ok(telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: "Choose your currency",
                        reply_markup: Some(&reply_markup),
                        ..Default::default()
                    })
                    .await?)
            }
        }
    }
}
