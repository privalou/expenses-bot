use crate::bot::dialogs::{Dialog, Feedback};
use crate::bot::error::BotError;
use crate::telegram::client::TelegramClient;
use crate::telegram::types::Message;

const HELP_TEXT: &str = r#"
You can send me these commands:
/start
/stop
/sendnow
/feedback
/help

If you encounter any issues feel free to open an issue.
Or you can also send feedback via /feedback command.
"#;

pub async fn start(telegram_client: &TelegramClient, user_id: &str) -> Result<(), BotError> {
    telegram_client
        .send_message(&Message {
            chat_id: user_id,
            text: HELP_TEXT,
            ..Default::default()
        })
        .await?;
    Ok(())
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
    telegram_client: &TelegramClient,
    author_id: &str,
    user_id: &str,
) -> Result<(), BotError> {
    Dialog::<Feedback>::new(user_id.to_string())
        .handle_current_step(&telegram_client, author_id, "")
        .await?;

    Ok(())
}

pub async fn send_now(telegram_client: &TelegramClient, user_id: &str) -> Result<(), BotError> {
    telegram_client
        .send_message(&Message {
            chat_id: user_id,
            text: "You haven't start using app. Start using /start command.",
            ..Default::default()
        })
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
    use serial_test::serial;

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
    #[serial]
    async fn feedback_success() {
        let url = &server_url();
        let message = Message {
            chat_id: USER_ID,
            text: FEEDBACK_TEXT,
            ..Default::default()
        };
        let _m = mock_send_message_success(TOKEN, &message);
        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));

        feedback(&telegram_client, "", USER_ID).await.unwrap();

        _m.assert();
    }

    #[tokio::test]
    #[serial]
    async fn send_now_success() {
        let url = &server_url();
        let message = Message {
            chat_id: USER_ID,
            text: "You haven't start using app. Start using /start command.",
            ..Default::default()
        };
        let _m1 = mock_send_message_success(TOKEN, &message);
        let telegram_client = TelegramClient::new_with(String::from(TOKEN), String::from(url));

        send_now(&telegram_client, USER_ID).await.unwrap();
        _m1.assert();
    }

    #[tokio::test]
    #[serial]
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
