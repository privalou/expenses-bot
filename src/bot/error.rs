use diesel::result::Error as DatabaseError;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use crate::telegram::error::TelegramError;
use r2d2::Error as DatabaseConnectionError;
use std::num::ParseFloatError;

#[derive(Debug)]
pub enum BotError {
    TelegramError(TelegramError),
    CustomError(String),
    DatabaseError(DatabaseError),
    ParsingError(ParseFloatError),
    DatabaseConnectionError(DatabaseConnectionError),
}

impl From<TelegramError> for BotError {
    fn from(error: TelegramError) -> Self {
        BotError::TelegramError(error)
    }
}

impl From<DatabaseError> for BotError {
    fn from(error: DatabaseError) -> Self {
        BotError::DatabaseError(error)
    }
}

impl From<String> for BotError {
    fn from(error_text: String) -> Self {
        BotError::CustomError(error_text)
    }
}

impl From<ParseFloatError> for BotError {
    fn from(parse_float_error: ParseFloatError) -> Self {
        BotError::ParsingError(parse_float_error)
    }
}

impl From<DatabaseConnectionError> for BotError {
    fn from(r2d2_error: DatabaseConnectionError) -> Self {
        BotError::DatabaseConnectionError(r2d2_error)
    }
}

impl Error for BotError {}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BotError::TelegramError(err) => err.fmt(f),
            BotError::DatabaseError(err) => err.fmt(f),
            BotError::CustomError(err) => err.fmt(f),
            BotError::ParsingError(err) => err.fmt(f),
            BotError::DatabaseConnectionError(err) => err.fmt(f),
        }
    }
}
