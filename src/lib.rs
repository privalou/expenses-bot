use crate::bot::api::init_bot;
use crate::bot::error::BotError;

pub mod bot;
mod log;
pub mod store;
pub mod telegram;

pub async fn start(tg_token: String) -> Result<(), BotError> {
    init_bot(&tg_token).await;
    Ok(())
}
