use std::env;

use expenses::bot::api::handle_message;
use expenses::telegram::client::TelegramClient;

#[tokio::test]
async fn integration_test_handle_message_start_flow() {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let telegram_client = TelegramClient::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN environment variable"),
    );
    let author_id = env::var("TELEGRAM_AUTHOR").expect("Set TELEGRAM_AUTHOR environment variable");
    let user_id =
        env::var("TELEGRAM_CHANNEL_ID").expect("Set TELEGRAM_CHANNEL_ID environment variable");
    handle_message(
        &telegram_client,
        author_id.as_str(),
        "/start".to_string(),
        user_id.clone(),
    )
    .await
    .unwrap();
    handle_message(
        &telegram_client,
        author_id.as_str(),
        "€".to_string(),
        user_id.clone(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn integration_test_handle_message_feedback_flow() {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let telegram_client = TelegramClient::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN environment variable"),
    );
    let author_id = env::var("TELEGRAM_AUTHOR").expect("Set TELEGRAM_AUTHOR environment variable");
    let user_id =
        env::var("TELEGRAM_CHANNEL_ID").expect("Set TELEGRAM_CHANNEL_ID environment variable");
    handle_message(
        &telegram_client,
        author_id.as_str(),
        "/feedback".to_string(),
        user_id.clone(),
    )
    .await
    .unwrap();
    handle_message(
        &telegram_client,
        author_id.as_str(),
        "Fooo".to_string(),
        user_id.clone(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn integration_test_handle_message_help_flow() {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let telegram_client = TelegramClient::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN environment variable"),
    );
    let author_id = env::var("TELEGRAM_AUTHOR").expect("Set TELEGRAM_AUTHOR environment variable");
    let user_id =
        env::var("TELEGRAM_CHANNEL_ID").expect("Set TELEGRAM_CHANNEL_ID environment variable");
    handle_message(
        &telegram_client,
        author_id.as_str(),
        "/help".to_string(),
        user_id.clone(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn integration_test_handle_message_sendnow_flow() {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let telegram_client = TelegramClient::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN environment variable"),
    );
    let author_id = env::var("TELEGRAM_AUTHOR").expect("Set TELEGRAM_AUTHOR environment variable");
    let user_id =
        env::var("TELEGRAM_CHANNEL_ID").expect("Set TELEGRAM_CHANNEL_ID environment variable");
    handle_message(
        &telegram_client,
        author_id.as_str(),
        "/sendnow".to_string(),
        user_id.clone(),
    )
    .await
    .unwrap();
}
