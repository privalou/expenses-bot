use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use crate::store::error::StoreError;
use crate::telegram::error::TelegramError;
use std::num::ParseFloatError;

#[derive(Debug)]
pub enum BotError {
    TelegramError(TelegramError),
    AnotherError(String),
    StoreError(StoreError),
    ParsingError(ParseFloatError),
}

impl From<TelegramError> for BotError {
    fn from(error: TelegramError) -> Self {
        BotError::TelegramError(error)
    }
}

impl From<StoreError> for BotError {
    fn from(error: StoreError) -> Self {
        BotError::StoreError(error)
    }
}

impl From<String> for BotError {
    fn from(error_text: String) -> Self {
        BotError::AnotherError(error_text)
    }
}

impl From<ParseFloatError> for BotError {
    fn from(parse_float_error: ParseFloatError) -> Self {
        BotError::ParsingError(parse_float_error)
    }
}

impl Error for BotError {}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BotError::TelegramError(err) => err.fmt(f),
            BotError::StoreError(err) => err.fmt(f),
            BotError::AnotherError(err) => err.fmt(f),
            BotError::ParsingError(err) => err.fmt(f),
        }
    }
}
