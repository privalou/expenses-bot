use crate::bot::api::init_bot;
use crate::bot::error::BotError;

mod bot;
mod logger;
mod telegram;

pub async fn start(tg_token: String, bot_name: String, author_id: String) -> Result<(), BotError> {
    init_bot(&tg_token, &bot_name, &author_id).await;
    Ok(())
}
