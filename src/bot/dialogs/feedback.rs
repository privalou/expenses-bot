use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;

use crate::bot::dialogs::Dialog;
use crate::bot::error::BotError;
use crate::store::simple_store::AppStore;
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
    pub fn new() -> Self {
        Dialog {
            command: "/feedback".to_string(),
            current_step: Feedback::Start,
            data: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn new_with(current_step: Feedback) -> Self {
        Dialog {
            command: "/feedback".to_string(),
            current_step,
            data: HashMap::new(),
        }
    }

    pub async fn handle_current_step(
        &mut self,
        store: &mut AppStore,
        telegram_client: &TelegramClient,
        user_id: &str,
        payload: &str,
    ) -> Result<(), BotError> {
        self.data.insert(self.current_step, payload.to_string());

        match self.current_step {
            Feedback::Start => {
                self.current_step = Feedback::Input;
                store.update_dialog(self.clone().into(), &user_id);
                telegram_client
                    .send_message(&Message {
                        chat_id: &user_id,
                        text: FEEDBACK_TEXT,
                        ..Default::default()
                    })
                    .await?;
            }
            Feedback::Input => {
                let input = self.data.get(&Feedback::Input).unwrap();
                info!("received feedback from user({}): {}", &user_id, input);
                store.update_dialog(self.clone().into(), &user_id);

                telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: &format!("Thanks, {}, for you priceless feedback!", &user_id),
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
    use mockito::server_url;

    use crate::store::simple_store::AppStore;
    use crate::telegram::test_helpers::mock_send_message_success;

    use super::*;

    const TOKEN: &str = "token";
    const USER_ID: &str = "123";

    const FEEDBACK_TEXT: &str = r#"
You can write your feedback. If you want the author to get back to you, leave your email.
Or you can contact the author via telegram: @privalou
Übermensch appoach is creating issue at https://github.com/privalou/expenses-bot
"#;

    #[tokio::test]
    async fn handle_current_step_success_start() {
        let mut store = AppStore::new();
        let url = &server_url();
        let feedback_command_message = Message {
            chat_id: USER_ID,
            text: FEEDBACK_TEXT,
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &feedback_command_message);

        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));
        let mut dialog = Dialog::<Feedback>::new();
        assert_eq!(
            dialog
                .handle_current_step(&mut store, &telegram_client, USER_ID, "")
                .await
                .expect("Can not process feedback step"),
            ()
        );

        mock.assert();
    }

    #[tokio::test]
    async fn handle_current_step_success_input() {
        let mut store = AppStore::new();

        let url = &server_url();
        let message_text = format!("Thanks, {}, for you priceless feedback!", USER_ID);
        let feedback_response_message = Message {
            chat_id: USER_ID,
            text: message_text.as_str(),
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &feedback_response_message);

        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));
        let mut dialog = Dialog::<Feedback>::new_with(Feedback::Input);
        assert_eq!(
            dialog
                .handle_current_step(&mut store, &telegram_client, USER_ID, "")
                .await
                .expect("Can not process feedback step"),
            ()
        );

        mock.assert();
    }
}
