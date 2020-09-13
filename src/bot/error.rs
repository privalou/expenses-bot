use crate::telegram::error::TelegramError;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum BotError {
    TelegramError(TelegramError),
}

impl From<TelegramError> for BotError {
    fn from(error: TelegramError) -> Self {
        BotError::TelegramError(error)
    }
}

impl Error for BotError {}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BotError::TelegramError(err) => err.fmt(f),
        }
    }
}
