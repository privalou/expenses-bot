use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use self::feedback::Feedback;
pub use self::start::Start;
use crate::store::simple_store::{DialogEntity, DialogPatch};

mod feedback;
mod start;

#[derive(Debug, Clone, PartialEq)]
pub struct Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    pub command: String,
    pub current_step: T,
    pub data: HashMap<T, String>,
}

impl<T> From<DialogEntity> for Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq + DeserializeOwned + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn from(dialog: DialogEntity) -> Self {
        Dialog {
            command: dialog.command,
            current_step: T::from_str(&dialog.step).unwrap(),
            data: serde_json::from_str(&dialog.data).unwrap(),
        }
    }
}

impl<T> Into<DialogPatch> for Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq + Serialize + std::string::ToString,
{
    fn into(self) -> DialogPatch {
        let command = match self.command.is_empty() {
            true => None,
            false => Some(self.command),
        };

        let step = match self.current_step.to_string().is_empty() {
            true => None,
            false => Some(self.current_step.to_string()),
        };

        let data = match self.data.is_empty() {
            true => None,
            false => Some(serde_json::to_string(&self.data).expect("Can not convert to data")),
        };

        DialogPatch {
            command,
            step,
            data,
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
        let dialog = dialog.clone();
        Dialog {
            command: dialog.command.to_string(),
            current_step: T::from_str(&dialog.step).expect("Can not convert to step"),
            data: serde_json::from_str(&dialog.data)
                .unwrap_or_else(|_| panic!("Can not convert to data: {}", &dialog.data)),
        }
    }
}

impl<T> Into<DialogEntity> for Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq + Serialize + std::string::ToString,
{
    fn into(self) -> DialogEntity {
        DialogEntity {
            command: self.command.clone(),
            step: self.current_step.to_string(),
            data: serde_json::to_string(&self.data).expect("Can not convert to data"),
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
                step: "FirstStep".to_string(),
                data: "{}".to_string(),
            }
        );
        let mut dialog_converted: Dialog<Start> = command.into();
        assert_eq!(dialog_converted, dialog);

        dialog_converted
            .data
            .insert(Start::FirstStep, "payload".to_string());
        dialog_converted.current_step = Start::Currency;
        let command_converted: DialogEntity = (dialog_converted.clone()).into();

        assert_eq!(
            command_converted,
            DialogEntity {
                command: "/start".to_string(),
                step: "Currency".to_string(),
                data: r#"{"FirstStep":"payload"}"#.to_string(),
            }
        )
    }
}
