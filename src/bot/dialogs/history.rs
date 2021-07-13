use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::{
    bot::{
        dialogs::{Command, Dialog},
        error::BotError,
    },
    db::{models::history::HistoryRepository, Connection},
    telegram::{client::TelegramClient, types::Message},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
pub enum History {
    List,
}

impl Default for Dialog<History> {
    fn default() -> Self {
        Self::new()
    }
}

impl Dialog<History> {
    pub fn new() -> Self {
        Dialog {
            command: Command::History,
            current_step: None,
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

        let records = HistoryRepository::get_all_records(user_id.to_string(), conn)?;

        let stringified_records = records
            .iter()
            .map(|a| a.print())
            .collect::<Vec<String>>()
            .join(", \n");

        telegram_client
            .send_message(&Message {
                chat_id: user_id,
                text: &stringified_records,
                ..Default::default()
            })
            .await
            .map_err(|e| BotError::TelegramError(e))
    }
}
