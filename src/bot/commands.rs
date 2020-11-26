use crate::bot::dialogs::{Dialog, Feedback, Start};
use crate::bot::error::BotError;
use crate::store::simple_store::AppStore;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

const HELP_TEXT: &str = r#"
You can send me these commands:
/start
/stop
/feedback
/help

If you encounter any issues feel free to open an issue.
Or you can also send feedback via /feedback command.
"#;

pub async fn start(
    store: &mut AppStore,
    telegram_client: &TelegramClient,
    user_id: &str,
) -> Result<(), BotError> {
    match Dialog::<Start>::new()
        .handle_current_step(store, &telegram_client, user_id, "")
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn stop(telegram_client: &TelegramClient, user_id: &str) -> Result<(), BotError> {
    telegram_client
        .send_message(&Message {
            chat_id: user_id,
            text: "User and subscriptions deleted",
            ..Default::default()
        })
        .await?;

    Ok(())
}

pub async fn feedback(
    store: &mut AppStore,
    telegram_client: &TelegramClient,
    user_id: &str,
) -> Result<(), BotError> {
    Dialog::<Feedback>::new()
        .handle_current_step(store, &telegram_client, user_id, "")
        .await?;

    Ok(())
}

pub async fn help(telegram_client: &TelegramClient, user_id: &str) -> Result<(), BotError> {
    telegram_client
        .send_message(&Message {
            chat_id: user_id,
            text: HELP_TEXT,
            ..Default::default()
        })
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use mockito::server_url;

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
    async fn feedback_success() {
        let mut store = AppStore::new();

        let url = &server_url();
        let message = Message {
            chat_id: USER_ID,
            text: FEEDBACK_TEXT,
            ..Default::default()
        };
        let _m = mock_send_message_success(TOKEN, &message);
        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));

        feedback(&mut store, &telegram_client, USER_ID)
            .await
            .unwrap();

        _m.assert();
    }

    #[tokio::test]
    async fn help_success() {
        let url = &server_url();
        let message = Message {
            chat_id: "123",
            text: HELP_TEXT,
            ..Default::default()
        };
        let _m = mock_send_message_success(TOKEN, &message);
        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));

        help(&telegram_client, "123").await.unwrap();
        _m.assert();
    }
}
