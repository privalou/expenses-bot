use std::env;

use expenses::bot::api::handle_message;
use expenses::store::simple_store::AppStore;
use expenses::telegram::client::TelegramClient;

#[tokio::test]
async fn integration_test_handle_message_start_flow() {
    let mut app_store = AppStore::new();
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let telegram_client = TelegramClient::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN environment variable"),
    );
    let user_id = env::var("USER_ID").expect("Set USER_ID environment variable");

    handle_message(
        &mut app_store,
        &telegram_client,
        "/start".to_string(),
        &user_id,
    )
    .await
    .expect("something bad happened");
    handle_message(&mut app_store, &telegram_client, "€".to_string(), &user_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn integration_test_handle_message_feedback_flow() {
    let mut app_store = AppStore::new();

    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let telegram_client = TelegramClient::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN environment variable"),
    );
    let user_id = env::var("USER_ID").expect("Set USER_ID environment variable");

    handle_message(
        &mut app_store,
        &telegram_client,
        "/feedback".to_string(),
        &user_id,
    )
    .await
    .unwrap();
    handle_message(
        &mut app_store,
        &telegram_client,
        "Fooo".to_string(),
        &user_id,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn integration_test_handle_message_help_flow() {
    let mut app_store = AppStore::new();

    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let telegram_client = TelegramClient::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN environment variable"),
    );
    let user_id = env::var("USER_ID").expect("Set USER_ID environment variable");

    handle_message(
        &mut app_store,
        &telegram_client,
        "/help".to_string(),
        &user_id,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn integration_test_handle_message_sendnow_flow() {
    let mut app_store = AppStore::new();

    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let telegram_client = TelegramClient::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN environment variable"),
    );
    let user_id = env::var("USER_ID").expect("Set USER_ID environment variable");

    handle_message(
        &mut app_store,
        &telegram_client,
        "/sendnow".to_string(),
        &user_id,
    )
    .await
    .unwrap();
}
