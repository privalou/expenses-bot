use std::fmt;
use std::str::FromStr;

use serde::de::DeserializeOwned;
use serde::export::Formatter;

use crate::db::models::dialog::DialogEntity;

pub use self::add::Add;
pub use self::feedback::Feedback;
pub use self::start::Start;

mod add;
mod feedback;
mod start;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Add,
    Start,
    Stop,
    Feedback,
    Help,
    History,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let printable = match self {
            Command::Start => "/start",
            Command::Stop => "/stop",
            Command::Feedback => "/feedback",
            Command::Help => "/help",
            Command::Add => "/add",
            Command::History => "/history",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    pub command: Command,
    pub current_step: Option<T>,
}

impl<T> From<DialogEntity> for Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq + DeserializeOwned + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn from(dialog: DialogEntity) -> Self {
        let command = match Command::from_str(&dialog.command) {
            Ok(command) => command,
            Err(_) => {
                panic!("Dialog command can not be parsed!")
            }
        };

        let current_step = match &dialog.step {
            Some(value) => match T::from_str(value) {
                Ok(value) => Some(value),
                Err(_) => None,
            },
            None => None,
        };
        Dialog {
            command,
            current_step,
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "/start" => Ok(Command::Start),
            "/stop" => Ok(Command::Stop),
            "/feedback" => Ok(Command::Feedback),
            "/help" => Ok(Command::Help),
            "/add" => Ok(Command::Add),
            "/history" => Ok(Command::History),
            _ => Err(()),
        }
    }
}

impl<T> From<&DialogEntity> for Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq + DeserializeOwned + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn from(dialog: &DialogEntity) -> Self {
        let command = match Command::from_str(&dialog.command) {
            Ok(command) => command,
            Err(_) => {
                panic!("Dialog command can not be parsed!")
            }
        };

        let current_step = match &dialog.step {
            Some(value) => match T::from_str(value) {
                Ok(value) => Some(value),
                Err(_) => None,
            },
            None => None,
        };
        Dialog {
            command,
            current_step,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_to_string() {
        assert_eq!(Command::Start.to_string(), "/start");
    }

    #[test]
    fn from_string_command() {
        let start_command: Command = "/start".parse().unwrap();
        assert_eq!(Command::Start, start_command);
    }

    #[test]
    fn error_at_parsing_invalid_command() {
        let command_result: Result<Command, ()> = "start".parse();
        assert!(command_result.is_err());
    }

    #[test]
    fn invalid_current_step_is_none() {
        let entity: DialogEntity = DialogEntity::new(
            "user_id".to_string(),
            "/start".to_string(),
            Some("foo".to_string()),
        );

        let dialog_converted: Dialog<Start> = entity.into();

        assert_eq!(
            dialog_converted,
            Dialog {
                command: Command::Start,
                current_step: None,
            }
        )
    }

    #[test]
    fn conversion_works_with_current_step_none() {
        let entity: DialogEntity =
            DialogEntity::new("user_id".to_string(), "/start".to_string(), None);

        let dialog_converted: Dialog<Start> = entity.into();

        assert_eq!(
            dialog_converted,
            Dialog {
                command: Command::Start,
                current_step: None,
            }
        )
    }

    #[test]
    fn conversion_works_with_current_step_some() {
        let entity: DialogEntity = DialogEntity::new(
            "user_id".to_string(),
            "/start".to_string(),
            Some("CurrencySelection".to_string()),
        );

        let dialog_converted: Dialog<Start> = entity.into();

        assert_eq!(
            dialog_converted,
            Dialog {
                command: Command::Start,
                current_step: Some(Start::CurrencySelection),
            }
        )
    }
}
