use diesel::result::Error as DatabaseError;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use crate::telegram::error::TelegramError;
use diesel::r2d2;
use std::num::ParseFloatError;

#[derive(Debug)]
pub enum BotError {
    TelegramError(TelegramError),
    CustomError(String),
    DatabaseError(DatabaseError),
    ParsingError(ParseFloatError),
    DatabaseConnectionError(r2d2::Error),
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

impl From<r2d2::Error> for BotError {
    fn from(r2d2_error: r2d2::Error) -> Self {
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
