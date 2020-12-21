use crate::bot::api::Bot;
use crate::bot::error::BotError;

pub mod bot;
mod log;
pub mod store;
pub mod telegram;

pub async fn start(tg_token: String) -> Result<(), BotError> {
    let bot = Bot::new(&tg_token);
    bot.init_bot(&tg_token).await;
    Ok(())
}
