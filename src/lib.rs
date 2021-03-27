#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::bot::error::BotError;
use crate::bot::Bot;

pub mod bot;
pub mod db;
mod log;
pub mod telegram;

pub async fn start(tg_token: String, db_url: String) -> Result<(), BotError> {
    let bot = Bot::new(&tg_token, &db_url);
    bot.init_bot().await;
    Ok(())
}
