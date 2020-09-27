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

    #[allow(dead_code)]
    pub fn new_with(user_id: String, current_step: Feedback) -> Self {
        Dialog {
            command: "/feedback".to_string(),
            user_id,
            current_step,
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
                        text: &format!("Thanks, {}, for you priceless feedback!", &self.user_id),
                        ..Default::default()
                    })
                    .await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::telegram::test_helpers::mock_send_message_success;
    use mockito::server_url;

    const TOKEN: &str = "token";
    const USER_ID: &str = "123";

    const FEEDBACK_TEXT: &str = r#"
You can write your feedback. If you want the author to get back to you, leave your email.
Or you can contact the author via telegram: @privalou
Übermensch appoach is creating issue at https://github.com/privalou/expenses-bot
"#;
    #[tokio::test]
    async fn handle_current_step_success_start() {
        let url = &server_url();
        let feedback_command_message = Message {
            chat_id: USER_ID,
            text: FEEDBACK_TEXT,
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &feedback_command_message);

        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));
        let mut dialog = Dialog::<Feedback>::new(String::from(USER_ID));
        dialog
            .handle_current_step(&telegram_client, USER_ID, "")
            .await
            .unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn handle_current_step_success_input() {
        let url = &server_url();
        let message_text = format!("Thanks, {}, for you priceless feedback!", USER_ID);
        let feedback_response_message = Message {
            chat_id: USER_ID,
            text: message_text.as_str(),
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &feedback_response_message);

        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));
        let mut dialog = Dialog::<Feedback>::new_with(String::from(USER_ID), Feedback::Input);
        dialog
            .handle_current_step(&telegram_client, USER_ID, "")
            .await
            .unwrap();

        mock.assert();
    }
}
