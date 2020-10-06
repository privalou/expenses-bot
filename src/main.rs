use dotenv::dotenv;
use expenses::start;
use std::env;

mod logger;
mod telegram;

#[derive(Debug)]
struct Post {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    logger::init_logger().expect("Can not run logging!");

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("Missing TELEGRAM_BOT_TOKEN env var");
    let author_id = env::var("TELEGRAM_AUTHOR").expect("Missing TELEGRAM_AUTHOR env var");
    let bot_name = env::var("BOT_NAME").expect("Missing BOT_NAME env var");

    start(token, bot_name, author_id).await?;

    Ok(())
}
