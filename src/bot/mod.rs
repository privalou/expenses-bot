use futures::StreamExt;
use log::{error, info};
use telegram_bot::{Api, MessageKind, MessageOrChannelPost, UpdateKind};

use crate::bot::dialogs::{Add, Dialog, Feedback, Start};
use crate::bot::error::BotError;
use crate::store::Store;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

pub mod dialogs;
pub mod error;

const ERROR_TEXT: &str = r#"
Looks like I'm having a technical glitch. Something went wrong.
If the issues persist send feedback via /feedback command.
"#;

const HELP_TEXT: &str = r#"
You can send me these commands:
/start
/stop
/feedback
/help
/add

If you encounter any issues feel free to open an issue.
Or you can also send feedback via /feedback command.
"#;

pub struct Bot {
    store: Store,
    telegram_client: TelegramClient,
}

impl Bot {
    pub fn new(token: &str) -> Self {
        let store = Store::new();
        let telegram_client = TelegramClient::new(token.to_string());
        Bot {
            store,
            telegram_client,
        }
    }

    pub fn new_with(store: Store, telegram_client: TelegramClient) -> Self {
        Bot {
            store,
            telegram_client,
        }
    }

    pub async fn init_bot(&self, token: &str) {
        let api = Api::new(&token);
        let mut stream = api.stream();
        while let Some(update) = stream.next().await {
            if let Ok(update) = update {
                match update.kind {
                    UpdateKind::Message(message) => {
                        if let MessageKind::Text { data, .. } = message.kind {
                            let user_id = message.from.id.to_string();
                            if let Err(e) = self.handle_message(data, &user_id).await {
                                error!("error handling message: {}", e);
                                self.telegram_client
                                    .send_message(&Message {
                                        chat_id: &user_id,
                                        text: ERROR_TEXT,
                                        ..Default::default()
                                    })
                                    .await
                                    .ok();
                            }
                        }
                    }
                    UpdateKind::CallbackQuery(query) => {
                        if query.message.is_none() {
                            info!("empty message in callback query");
                            continue;
                        }

                        if query.data.is_none() {
                            info!("empty data in callback query");
                            continue;
                        }
                        let message = query
                            .message
                            .expect("There is no message at callback query");
                        let data = query.data.expect("There is no data at callback query");
                        let user_id = match message {
                            MessageOrChannelPost::Message(message) => message.chat.id().to_string(),
                            MessageOrChannelPost::ChannelPost(post) => post.chat.id.to_string(),
                        };

                        if let Err(e) = self.handle_message(data, &user_id).await {
                            error!("error handling message: {}", e);
                            let error_message = Message {
                                chat_id: &user_id,
                                text: ERROR_TEXT,
                                ..Default::default()
                            };
                            self.telegram_client.send_message(&error_message).await.ok();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub async fn handle_message(&self, payload: String, user_id: &str) -> Result<String, BotError> {
        info!("received message from: {}, message: {}", user_id, payload);

        // TODO: Extract commands as enum
        let sent_text_message = match payload.as_ref() {
            "/start" => {
                Dialog::<Start>::new()
                    .handle_current_step(&self.store, &self.telegram_client, user_id, "")
                    .await?
            }
            "/stop" => {
                let sent_text = &self
                    .telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: "User and subscriptions deleted",
                        ..Default::default()
                    })
                    .await?;
                sent_text.to_owned()
            }
            "/feedback" => {
                Dialog::<Feedback>::new()
                    .handle_current_step(&self.store, &self.telegram_client, user_id, "")
                    .await?
            }
            "/help" => {
                let sent_text = &self
                    .telegram_client
                    .send_message(&Message {
                        chat_id: user_id,
                        text: HELP_TEXT,
                        ..Default::default()
                    })
                    .await?;
                sent_text.to_owned()
            }
            "/add" => {
                Dialog::<Add>::new()
                    .handle_current_step(&self.store, &self.telegram_client, user_id, "")
                    .await?
            }
            _ => {
                handle_not_a_command_message(&self.store, &self.telegram_client, &user_id, &payload)
                    .await?
            }
        };
        Ok(sent_text_message)
    }
}

/// process if this message received from registered user else send don't get message
async fn handle_not_a_command_message(
    store: &Store,
    telegram_client: &TelegramClient,
    user_id: &str,
    payload: &str,
) -> Result<String, BotError> {
    match store.get_user_dialog(user_id) {
        Ok(dialog_from_store) => {
            if let Some(dialog_entity) = dialog_from_store {
                let sent_message = match dialog_entity.command.as_str() {
                    "/start" => {
                        let dialog: Dialog<Start> = Dialog::from(dialog_entity);
                        dialog
                            .handle_current_step(store, telegram_client, user_id, payload)
                            .await?
                    }
                    "/feedback" => {
                        let mut dialog: Dialog<Feedback> = Dialog::from(dialog_entity);
                        dialog
                            .handle_current_step(store, telegram_client, user_id, payload)
                            .await?
                    }
                    _ => "".to_string(),
                };
                Ok(sent_message)
            } else {
                Err(BotError::AnotherError("Unrecognized message. None user dialog at store and not a command message".to_string()))
            }
        }
        Err(_) => {
            Ok(
                telegram_client
                    .send_message(&Message {
                        chat_id: &user_id,
                        text: "Only available for registered users. Use /help to see list of available commands.",
                        ..Default::default()
                    })
                    .await?)
        }
    }
}

#[cfg(test)]
mod test {
    use mockito::server_url;

    use crate::store::DialogEntity;
    use crate::telegram::test_helpers::mock_send_message_success;
    use crate::telegram::types::{
        InlineKeyboardButton, InlineKeyboardMarkup, Message, ReplyMarkup,
    };

    use super::*;

    const TOKEN: &str = "token";
    const USER_ID: &str = "123";

    const FEEDBACK_TEXT: &str =
        "You can write your feedback. If you want the author to get back to \
you, leave your email. Or you can contact the author via telegram: @privalou \
Übermensch appoach is creating issue at github.com/privalou/expenses-bot";

    /// This is a test for parsing /start command from a user who was not in a store before this
    /// message.
    /// As a result handle message should not throw an error and the store should have a new record
    /// about user and his dialogs.
    #[tokio::test]
    async fn handle_message_start_first_step() {
        //given

        let markup = ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
            inline_keyboard: vec![vec![
                InlineKeyboardButton::new("₽"),
                InlineKeyboardButton::new("$"),
                InlineKeyboardButton::new("€"),
            ]],
        });
        let start_first_step_success_action = Message {
            chat_id: &USER_ID,
            text: "Choose your currency",
            reply_markup: Some(&markup),
            ..Default::default()
        };

        let mock = mock_send_message_success(TOKEN, &start_first_step_success_action);

        let bot = configure_bot();
        let message = bot.handle_message("/start".to_string(), USER_ID).await;

        //expect
        assert_eq!(
            start_first_step_success_action.text,
            message.expect("Can not handle message")
        );
        let dialog_option = bot
            .store
            .get_user_dialog(&USER_ID)
            .expect("There is no user with such ID");
        assert!(dialog_option.is_some());
        let dialog = dialog_option.unwrap();
        assert_eq!("/start", dialog.command);
        assert_eq!("CurrencySelection", dialog.step.as_ref().unwrap());

        mock.assert();
    }

    #[tokio::test]
    async fn handle_message_currency_step() {
        //given
        let dialog =
            DialogEntity::new_with("/start".to_string(), Some("CurrencySelection".to_string()))
                .expect("Invalid DialogEntity");

        let start_end_step_message = Message {
            chat_id: USER_ID,
            text: "Your currency is €",
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &start_end_step_message);

        let store = Store::new();
        store.save_user(&USER_ID);
        assert!(store.update_dialog(Some(dialog), USER_ID).is_ok());

        let bot = Bot::new_with(
            store,
            TelegramClient::new_with(String::from(TOKEN), String::from(server_url())),
        );

        let sent_text_message = bot.handle_message("€".to_string(), USER_ID).await;

        let user_data = bot
            .store
            .get_user_data(USER_ID)
            .expect("No user with such id");

        assert_eq!(
            user_data
                .currency
                .clone()
                .expect("Currency is None after value persisted"),
            "€".to_string()
        ); //expect
        assert_eq!(
            start_end_step_message.text,
            sent_text_message.expect("Can not handle message")
        );

        mock.assert();
    }

    #[tokio::test]
    async fn feedback_for_registered_user_success() {
        let store = Store::new();
        store.save_user(USER_ID);

        let url = &server_url();
        let message = Message {
            chat_id: USER_ID,
            text: FEEDBACK_TEXT,
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &message);
        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));

        let bot = Bot::new_with(store, telegram_client);
        assert!(bot
            .handle_message("/feedback".to_string(), USER_ID)
            .await
            .is_ok());

        mock.assert();
    }

    #[tokio::test]
    async fn help_success() {
        let help_message = Message {
            chat_id: USER_ID,
            text: HELP_TEXT,
            ..Default::default()
        };
        let mock = mock_send_message_success(TOKEN, &help_message);
        let bot = configure_bot();

        let result = bot.handle_message("/help".to_string(), USER_ID).await;
        assert!(result.is_ok());
        assert_eq!(result.expect("Error in help"), "Successfully sent");
        mock.assert();
    }

    fn configure_bot() -> Bot {
        let store = Store::new();
        let telegram_client =
            TelegramClient::new_with(String::from(TOKEN), String::from(&server_url()));
        Bot::new_with(store, telegram_client)
    }
}
