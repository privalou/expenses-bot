use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::bot::dialogs::Dialog;
use crate::bot::error::BotError;
use crate::store::simple_store::Store;
use crate::telegram::client::TelegramClient;

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
        _store: &Store,
        _telegram_client: &TelegramClient,
        user_id: &str,
        payload: &str,
    ) -> Result<String, BotError> {
        info!("Received {} payload from user {}", payload, user_id);

        match self.current_step {
            Some(Add::Amount) => {}
            Some(Add::Category) => {}
            None => {}
        }
        Ok("foo".to_string())
    }
}
