use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;

use crate::bot::dialogs::{Command, Dialog};
use crate::bot::error::BotError;
use crate::db::models::dialog::DialogEntity;
use crate::db::Connection;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

const FEEDBACK_TEXT: &str = "You can write your feedback. If you want the author to get back to \
you, leave your email. Or you can contact the author via telegram: @privalou \
Übermensch appoach is creating issue at github.com/privalou/bot";

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
pub enum Feedback {
    Start,
    Input,
}

impl Dialog<Feedback> {
    pub fn new() -> Self {
        Dialog {
            command: Command::Feedback,
            current_step: Some(Feedback::Start),
        }
    }

    #[allow(dead_code)]
    pub fn new_with(current_step: Feedback) -> Self {
        Dialog {
            command: Command::Feedback,
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
        let step = match self.current_step {
            None => Feedback::Start,
            Some(step) => step,
        };
        match step {
            Feedback::Start => {
                let entity = DialogEntity::new(
                    user_id.to_string(),
                    Command::Feedback.to_string(),
                    Some(Feedback::Input.to_string()),
                );
                DialogEntity::update_dialog(&entity, conn)?;

                Ok(telegram_client
                    .send_message(&Message {
                        chat_id: &user_id,
                        text: FEEDBACK_TEXT,
                        ..Default::default()
                    })
                    .await?)
            }

            Feedback::Input => {
                info!("received feedback from user({}): {}", user_id, payload);
                let entity = DialogEntity::new(user_id.to_string(), self.command.to_string(), None);
                DialogEntity::update_dialog(&entity, conn)?;

                Ok(telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: &format!("Thanks, {}, for you priceless feedback!", &user_id),
                        ..Default::default()
                    })
                    .await?)
            }
        }
    }
}
