use bot::start;
use dotenv::dotenv;
use std::env;

mod log;
mod telegram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_var) = env::var("ENV") {
        println!("Recieved value from ENV param {}", &env_var,);
        match env_var.as_str() {
            "DEV" => dotenv::from_filename("dev.env")
                .expect("Failed to read env variables from test.env"),
            "PROD" => dotenv().expect("Failed to read .env file"),
            _ => dotenv().expect("Failed to read .env file"),
        };
    } else {
        println!("Recieved no value from ENV param");
        dotenv().ok();
    }

    log::logger::init_logger().expect("Can not run logging!");

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("Missing TELEGRAM_BOT_TOKEN env var");
    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env var");

    start(token, db_url).await?;

    Ok(())
}
