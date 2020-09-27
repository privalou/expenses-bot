use std::error::Error;
use std::fmt;

use reqwest::Error as ReqwestError;
use serde::export::Formatter;
use serde_json::error::Error as SerdeError;

#[derive(Debug)]
pub enum TelegramError {
    NetworkError(ReqwestError),
    MalformedResponse(SerdeError),
    Unsuccessful(String),
}

impl From<ReqwestError> for TelegramError {
    fn from(error: ReqwestError) -> Self {
        TelegramError::NetworkError(error)
    }
}

impl From<SerdeError> for TelegramError {
    fn from(error: SerdeError) -> Self {
        TelegramError::MalformedResponse(error)
    }
}

impl From<String> for TelegramError {
    fn from(error: String) -> Self {
        TelegramError::Unsuccessful(error)
    }
}

impl Error for TelegramError {}

impl fmt::Display for TelegramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TelegramError::NetworkError(err) => err.fmt(f),
            TelegramError::MalformedResponse(err) => err.fmt(f),
            TelegramError::Unsuccessful(err) => err.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::telegram::error::TelegramError;
    use serde::de::Error;
    use serde_json::error::Error as SerdeError;

    #[test]
    fn fmt_telegram_unsuccessful() {
        let error = TelegramError::Unsuccessful(String::from("unsuccessful sending"));
        assert_eq!(
            format!("Fail due to {}", error),
            "Fail due to unsuccessful sending"
        );
    }

    #[test]
    fn fmt_telegram_malformed_response() {
        let error = TelegramError::MalformedResponse(SerdeError::custom("malformed response"));
        assert_eq!(
            format!("Fail due to {}", error),
            "Fail due to malformed response"
        );
    }
}
