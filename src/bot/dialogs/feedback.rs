use log::info;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;

use crate::bot::dialogs::Dialog;
use crate::bot::error::BotError;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

const FEEDBACK_TEXT: &str = r#"
You can write your feedback. If you want the author to get back to you, leave your email.
Or you can contact the author via telegram: @privalou
Übermensch appoach is creating issue at https://github.com/privalou/expenses-bot
"#;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
pub enum Feedback {
    Start,
    Input,
}

impl Dialog<Feedback> {
    pub fn new(user_id: String) -> Self {
        Dialog {
            command: "/feedback".to_string(),
            user_id,
            current_step: Feedback::Start,
            data: HashMap::new(),
        }
    }

    pub async fn handle_current_step(
        &mut self,
        telegram_client: &TelegramClient,
        author_id: &str,
        payload: &str,
    ) -> Result<(), BotError> {
        self.data.insert(self.current_step, payload.to_string());

        match self.current_step {
            Feedback::Start => {
                self.current_step = Feedback::Input;

                telegram_client
                    .send_message(&Message {
                        chat_id: &self.user_id,
                        text: FEEDBACK_TEXT,
                        ..Default::default()
                    })
                    .await?;
            }
            Feedback::Input => {
                let input = self.data.get(&Feedback::Input).unwrap();
                info!("received feedback from user({}): {}", &self.user_id, input);

                telegram_client
                    .send_message(&Message {
                        chat_id: author_id,
                        text: &format!("Received input from user({}):\n{}", &self.user_id, input),
                        ..Default::default()
                    })
                    .await?;

                telegram_client
                    .send_message(&Message {
                        chat_id: &self.user_id,
                        text: "Passed your feedback to my creator. Thanks for the input!",
                        ..Default::default()
                    })
                    .await?;
            }
        }
        Ok(())
    }
}
