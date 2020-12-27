use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;

use crate::bot::dialogs::Dialog;
use crate::bot::error::BotError;
use crate::store::{DialogEntity, Store};
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

const FEEDBACK_TEXT: &str = "You can write your feedback. If you want the author to get back to \
you, leave your email. Or you can contact the author via telegram: @privalou \
Übermensch appoach is creating issue at github.com/privalou/expenses-bot";

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Display, EnumString)]
pub enum Feedback {
    Start,
    Input,
}

impl Dialog<Feedback> {
    pub fn new() -> Self {
        Dialog {
            command: "/feedback".to_string(),
            current_step: Some(Feedback::Start),
        }
    }

    #[allow(dead_code)]
    pub fn new_with(current_step: Feedback) -> Self {
        Dialog {
            command: "/feedback".to_string(),
            current_step: Some(current_step),
        }
    }

    pub async fn handle_current_step(
        &mut self,
        store: &Store,
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
                match store.update_dialog(
                    Some(
                        DialogEntity::new_with(
                            "/feedback".to_string(),
                            Some(Feedback::Input.to_string()),
                        )
                        .expect("Can not create such dialog"),
                    ),
                    &user_id,
                ) {
                    Ok(_) => {
                        let received_text = telegram_client
                            .send_message(&Message {
                                chat_id: &user_id,
                                text: FEEDBACK_TEXT,
                                ..Default::default()
                            })
                            .await?;
                        Ok(received_text)
                    }
                    Err(_) => {
                        let sent_text = telegram_client
                            .send_message(&Message {
                                chat_id: &user_id,
                                text: "Only registered users can leave a feedback. Go to /start and create profile before leaving a feedback",
                                ..Default::default()
                            })
                            .await?;
                        Ok(sent_text)
                    }
                }
            }

            Feedback::Input => {
                info!("received feedback from user({}): {}", user_id, payload);
                store
                    .update_dialog(None, &user_id)
                    .expect("Dialog was not saved");

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

#[cfg(test)]
mod test {
    use mockito::server_url;

    use crate::store::Store;
    use crate::telegram::test_helpers::mock_send_message_success;

    use super::*;

    const TOKEN: &str = "token";
    const USER_ID: &str = "123";

    const FEEDBACK_TEXT: &str =
        "You can write your feedback. If you want the author to get back to \
you, leave your email. Or you can contact the author via telegram: @privalou \
Übermensch appoach is creating issue at github.com/privalou/expenses-bot";

    #[tokio::test]
    async fn handle_start_step_with_registered_user() {
        let mut store = Store::new();
        store.save_user(USER_ID);

        let url = &server_url();
        let feedback_command_message = Message {
            chat_id: USER_ID,
            text: FEEDBACK_TEXT,
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &feedback_command_message);

        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));
        let mut dialog = Dialog::<Feedback>::new();
        let text_message = dialog
            .handle_current_step(&mut store, &telegram_client, USER_ID, "")
            .await
            .expect("Can not process feedback step");
        assert_eq!(text_message, FEEDBACK_TEXT);

        mock.assert();
    }

    #[tokio::test]
    async fn handle_current_step_success_input() {
        let mut store = Store::new();
        store.save_user(USER_ID);
        let dialog_entity =
            DialogEntity::new_with("/feedback".to_string(), Some("Feedback::Input".to_string()))
                .expect("Invalid");
        assert!(store.update_dialog(Some(dialog_entity), USER_ID).is_ok());

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
            message_text
        );

        mock.assert();
    }
}
