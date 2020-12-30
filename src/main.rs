use dotenv::dotenv;
use expenses::start;
use std::env;

mod log;
mod telegram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    log::logger::init_logger().expect("Can not run logging!");

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("Missing TELEGRAM_BOT_TOKEN env var");
    let db_url = env::var("TELEGRAM_BOT_TOKEN").expect("Missing TELEGRAM_BOT_TOKEN env var");

    start(token, db_url).await?;

    Ok(())
}
