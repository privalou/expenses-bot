use std::str::FromStr;

use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use futures::StreamExt;
use log::{error, info};
use telegram_bot::{Api, MessageKind, MessageOrChannelPost, UpdateKind};

use crate::bot::dialogs::{Add, Command, Dialog, Feedback, Start};
use crate::bot::error::BotError;
use crate::db::models::dialog::DialogEntity;
use crate::db::{migrate_and_config_db, Connection};
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

pub mod dialogs;
pub mod error;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

const ERROR_TEXT: &str = r#"
Looks like I'm having a technical glitch. Something went wrong.
If the issues persist send feedback via /feedback command.
"#;

const HELP_TEXT: &str = r#"
You can send me these commands:
/start
/feedback
/help
/add

If you encounter any issues feel free to open an issue.
Or you can also send feedback via /feedback command.
"#;

pub struct Bot {
    store_pool: Pool,
    telegram_client: TelegramClient,
}

impl Bot {
    pub fn new(token: &str, db_url: &str) -> Self {
        let store_pool = migrate_and_config_db(db_url);
        let telegram_client = TelegramClient::new(token.to_string());
        Bot {
            store_pool,
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
                                match e {
                                    BotError::ParsingError(_) => (),
                                    _ => {
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

        // todo: fix!
        let connection = self
            .store_pool
            .get()
            .expect("Can not get connection from pool.");

        let sent_text_message = match payload.as_ref() {
            "/start" => {
                Dialog::<Start>::new()
                    .handle_current_step(&connection, &self.telegram_client, user_id, "")
                    .await?
            }
            "/feedback" => {
                Dialog::<Feedback>::new()
                    .handle_current_step(&connection, &self.telegram_client, user_id, "")
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
                    .handle_current_step(&connection, &self.telegram_client, user_id, "")
                    .await?
            }
            _ => {
                handle_not_a_command_message(&connection, &self.telegram_client, &user_id, &payload)
                    .await?
            }
        };
        Ok(sent_text_message)
    }
}

/// process if this message received from registered user else send don't get message
async fn handle_not_a_command_message(
    conn: &Connection,
    telegram_client: &TelegramClient,
    user_id: &str,
    payload: &str,
) -> Result<String, BotError> {
    match DialogEntity::get_user_dialog(user_id, conn) {
        Ok(dialog_entity) => {
            let command = Command::from_str(&dialog_entity.command).expect("Can not process command. Problem with dialog entity probably");

            let sent_message = match command {
                Command::Start => {
                    let dialog: Dialog<Start> = dialog_entity.into();
                    dialog
                        .handle_current_step(conn, telegram_client, user_id, payload)
                        .await?
                }
                Command::Feedback => {
                    let dialog: Dialog<Feedback> = dialog_entity.into();
                    dialog
                        .handle_current_step(conn, telegram_client, user_id, payload)
                        .await?
                }
                Command::Add => {
                    let dialog: Dialog<Add> =dialog_entity.into();
                    dialog
                        .handle_current_step(conn, telegram_client, user_id, payload)
                        .await?
                }
                _ => { unimplemented!() }
            };
            Ok(sent_message)
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
