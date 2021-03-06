use std::fmt;
use std::str::FromStr;

use futures::StreamExt;
use log::{error, info};
use telegram_bot::{MessageKind, UpdateKind};

use crate::{
    bot::{
        dialogs::{Add, Command, Dialog, Feedback, History, Start},
        error::BotError,
    },
    db::{models::dialog::DialogEntity, Connection, DbConnectionPool},
    telegram::{client::TelegramClient, types::Message},
};

pub mod dialogs;
pub mod error;

const ERROR_TEXT: &str = r#"
Looks like I'm having a technical glitch. Something went wrong.
If the issues persist send feedback via /feedback command.
"#;

const HELP_TEXT: &str = r#"
You can send me these commands:
/start
/feedback
/help
/history
/add

If you encounter any issues feel free to open an issue.
Or you can also send feedback via /feedback command.
"#;

pub struct Bot {
    connection_pool: DbConnectionPool,
    telegram_client: TelegramClient,
}

impl fmt::Debug for Bot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.telegram_client)
    }
}

impl Bot {
    pub fn new(token: &str, db_url: &str) -> Self {
        let connection_pool = DbConnectionPool::new(db_url);
        let telegram_client = TelegramClient::new(token.to_string());
        Bot {
            connection_pool,
            telegram_client,
        }
    }

    pub async fn init_bot(&self) {
        let mut stream = self.telegram_client.stream();
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
                                        let _ = self
                                            .telegram_client
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
                        let data = query.data.expect("There is no data at callback query");
                        let user_id = query.from.id.to_string();

                        if let Err(e) = self.handle_message(data, &user_id).await {
                            error!("error handling message: {}", e);
                            let error_message = Message {
                                chat_id: &user_id,
                                text: ERROR_TEXT,
                                ..Default::default()
                            };
                            let _ = self.telegram_client.send_message(&error_message).await.ok();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub async fn handle_message(&self, payload: String, user_id: &str) -> Result<String, BotError> {
        info!("received message from: {}, message: {}", user_id, payload);

        let connection = self.connection_pool.establish_connection();

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
            "/history" => {
                Dialog::<History>::new()
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

            match command {
                Command::Start => {
                    let dialog: Dialog<Start> = dialog_entity.into();
                    Ok(dialog
                        .handle_current_step(conn, telegram_client, user_id, payload)
                        .await?)
                }
                Command::Feedback => {
                    let dialog: Dialog<Feedback> = dialog_entity.into();
                    Ok(dialog
                        .handle_current_step(conn, telegram_client, user_id, payload)
                        .await?)
                }
                Command::Add => {
                    let dialog: Dialog<Add> = dialog_entity.into();
                    Ok(dialog
                        .handle_current_step(conn, telegram_client, user_id, payload)
                        .await?)
                }
                _ => {Err(BotError::UnrecognisedCommand("can not process such command".to_string()))}
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
