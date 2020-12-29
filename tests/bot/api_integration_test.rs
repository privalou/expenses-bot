use std::env;

use diesel::{Connection, PgConnection};

use expenses::bot::Bot;
use expenses::db::clear_tables;

#[tokio::test]
async fn integration_test_handle_message_start_flow() {
    let bot = configure_bot();
    let user_id = env::var("USER_ID").expect("Set USER_ID environment variable");
    let response_for_start = bot
        .handle_message("/start".to_string(), &user_id)
        .await
        .unwrap();
    assert_eq!("Choose your currency".to_string(), response_for_start);
    let response_for_currency = bot.handle_message("€".to_string(), &user_id).await.unwrap();
    assert_eq!("Your currency is €".to_string(), response_for_currency);
    clean_up();
}

#[tokio::test]
async fn integration_test_handle_message_feedback_flow() {
    let bot = configure_bot();
    let user_id = env::var("USER_ID").expect("Set USER_ID environment variable");

    bot.handle_message("/feedback".to_string(), &user_id)
        .await
        .unwrap();
    bot.handle_message("Fooo".to_string(), &user_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn integration_test_handle_message_help_flow() {
    let bot = configure_bot();
    let user_id = env::var("USER_ID").expect("Set USER_ID environment variable");

    bot.handle_message("/help".to_string(), &user_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn integration_test_handle_message_send_now_flow() {
    let bot = configure_bot();
    let user_id = env::var("USER_ID").expect("Set USER_ID environment variable");

    bot.handle_message("/sendnow".to_string(), &user_id)
        .await
        .unwrap();
}

fn configure_bot() -> Bot {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    clean_up();
    Bot::new(
        &env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN environment variable"),
        &env::var("DATABASE_URL")
            .expect("Set DATABASE_URL environment variable or configure it at test.env file"),
    )
}

fn clean_up() {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let db_url = env::var("DATABASE_URL")
        .expect("Set DATABASE_URL environment variable or configure it at test.env file");
    let connection = PgConnection::establish(&db_url).unwrap();
    clear_tables(&connection);
}
