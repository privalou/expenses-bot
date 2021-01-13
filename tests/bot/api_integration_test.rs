use std::env;

use diesel::{Connection, PgConnection};

use expenses::bot::Bot;
use expenses::db::clear_tables;

#[tokio::test]
async fn full_commands_integration_flow() {
    let bot = configure_bot();
    let user_id = env::var("USER_ID").expect("Set USER_ID environment variable");

    let response_for_help_message = bot
        .handle_message("/help".to_string(), &user_id)
        .await
        .unwrap();
    let help_message = r#"You can send me these commands:
/start
/feedback
/help
/add

If you encounter any issues feel free to open an issue.
Or you can also send feedback via /feedback command."#
        .to_string();
    assert_eq!(help_message, response_for_help_message);

    let response = bot
        .handle_message("/start".to_string(), &user_id)
        .await
        .unwrap();
    assert_eq!("Choose your currency".to_string(), response);
    let response = bot.handle_message("€".to_string(), &user_id).await.unwrap();
    assert_eq!("Your currency is €".to_string(), response);

    let response = bot
        .handle_message("/feedback".to_string(), &user_id)
        .await
        .unwrap();

    assert_eq!("You can write your feedback. If you want the author to get back to you, leave your email. Or you can contact the author via telegram: @privalou Übermensch appoach is creating issue at github.com/privalou/expenses-bot".to_string(), response);

    let response = bot
        .handle_message("Fooo".to_string(), &user_id)
        .await
        .unwrap();

    assert_eq!(
        "Thanks, 54981987, for you priceless feedback!".to_string(),
        response
    );

    clean_up();
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

pub fn clean_up() {
    let connection = establish_connection();
    clear_tables(&connection);
}

fn establish_connection() -> PgConnection {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let db_url = env::var("DATABASE_URL")
        .expect("Set DATABASE_URL environment variable or configure it at test.env file");
    PgConnection::establish(&db_url).unwrap()
}
