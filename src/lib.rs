#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    private_doc_tests,
    trivial_casts,
    trivial_numeric_casts,
    unused,
    future_incompatible,
    nonstandard_style,
    unsafe_code,
    unused_import_braces,
    unused_results,
    variant_size_differences
)]

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
