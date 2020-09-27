use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumString};

use crate::bot::dialogs::Dialog;
use crate::bot::error::BotError;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, ReplyMarkup};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
pub enum Start {
    FirstStep,
    Currency,
    End,
}

impl Dialog<Start> {
    pub fn new(user_id: String) -> Self {
        Dialog {
            command: "/start".to_string(),
            user_id,
            current_step: Start::FirstStep,
            data: HashMap::new(),
        }
    }

    pub fn new_with(user_id: String, current_step: Start) -> Self {
        Dialog {
            command: "/start".to_string(),
            user_id,
            current_step,
            data: HashMap::new(),
        }
    }

    pub async fn handle_current_step(
        &mut self,
        telegram_client: &TelegramClient,
        user_id: &str,
        payload: &str,
    ) -> Result<(), BotError> {
        self.data.insert(self.current_step, payload.to_string());

        match self.current_step {
            Start::FirstStep => {
                self.current_step = Start::Currency;
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
                let reply_markup =
                    ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup { inline_keyboard });
                telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: "Choose your currency",
                        reply_markup: Some(&reply_markup),
                        ..Default::default()
                    })
                    .await?;
            }
            Start::Currency => {
                self.current_step = Start::End;
                info!(
                    "received response at Currency step {}",
                    self.data
                        .get(&Start::Currency)
                        .expect("ERROR AT CURRENCY RETRIEVING")
                );
                telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: format!("Your currency is {}", payload).as_str(),
                        ..Default::default()
                    })
                    .await?;
            }
            Start::End => info!("fook"),
        }
        Ok(())
    }
}
