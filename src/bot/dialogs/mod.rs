use serde::de::DeserializeOwned;

use crate::store::simple_store::DialogEntity;

pub use self::feedback::Feedback;
pub use self::start::Start;
use serde::Serialize;

mod add;
mod feedback;
mod start;

#[derive(Debug, Clone, PartialEq)]
pub struct Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    pub command: String,
    pub current_step: Option<T>,
}

impl<T> From<DialogEntity> for Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq + DeserializeOwned + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn from(dialog: DialogEntity) -> Self {
        let current_step = match &dialog.step {
            Some(value) => match T::from_str(value) {
                Ok(value) => Some(value),
                Err(_) => None,
            },
            None => None,
        };
        Dialog {
            command: dialog.command,
            current_step,
        }
    }
}

// idk how stupid it is
impl<T> From<&DialogEntity> for Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq + DeserializeOwned + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn from(dialog: &DialogEntity) -> Self {
        let current_step = match &dialog.step {
            Some(value) => match T::from_str(value) {
                Ok(value) => Some(value),
                Err(_) => None,
            },
            None => None,
        };
        Dialog {
            command: dialog.command.to_string(),
            current_step,
        }
    }
}

impl<T> Into<DialogEntity> for Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq + Serialize + std::string::ToString,
{
    fn into(self) -> DialogEntity {
        let step = match self.current_step {
            None => None,
            Some(value) => Some(value.to_string()),
        };

        DialogEntity {
            command: self.command,
            step,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_works() {
        let dialog = Dialog::<Start>::new();
        // TODO: is there some way to do the conversion without cloning?
        let command: DialogEntity = (dialog.clone()).into();

        assert_eq!(
            command,
            DialogEntity {
                command: "/start".to_string(),
                step: None,
            }
        );
        let mut dialog_converted: Dialog<Start> = command.into();
        assert_eq!(dialog_converted, dialog);

        dialog_converted.current_step = Some(Start::CurrencySelection);
        let command_converted: DialogEntity = (dialog_converted.clone()).into();

        assert_eq!(
            command_converted,
            DialogEntity {
                command: "/start".to_string(),
                step: Some("CurrencySelection".to_string()),
            }
        )
    }
}
