use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use self::feedback::Feedback;
use crate::bot::dialogs::entity::DialogEntity;

mod entity;
mod feedback;

#[derive(Debug, Clone, PartialEq)]
pub struct Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    pub command: String,
    pub user_id: String,
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
            user_id: dialog.user_id,
            command: dialog.command,
            current_step: T::from_str(&dialog.step).unwrap(),
            data: serde_json::from_str(&dialog.data).unwrap(),
        }
    }
}

impl<T> Into<DialogEntity> for Dialog<T>
where
    T: std::hash::Hash + std::cmp::Eq + Serialize + std::string::ToString,
{
    fn into(self) -> DialogEntity {
        DialogEntity {
            user_id: self.user_id.clone(),
            command: self.command.clone(),
            step: self.current_step.to_string(),
            data: serde_json::to_string(&self.data).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_works() {
        let dialog = Dialog::<Feedback>::new("123".to_string());
        // TODO: is there some way to do the conversion without cloning?
        let command: DialogEntity = (dialog.clone()).into();

        assert_eq!(
            command,
            DialogEntity {
                user_id: "123".to_string(),
                command: "/feedback".to_string(),
                step: "Start".to_string(),
                data: "{}".to_string(),
            }
        );
        let mut dialog_converted: Dialog<Feedback> = command.into();
        assert_eq!(dialog_converted, dialog);

        dialog_converted
            .data
            .insert(Feedback::Start, "payload".to_string());
        dialog_converted.current_step = Feedback::Start;
        let command_converted: DialogEntity = (dialog_converted.clone()).into();

        assert_eq!(
            command_converted,
            DialogEntity {
                user_id: "123".to_string(),
                command: "/feedback".to_string(),
                step: "Start".to_string(),
                data: r#"{"Start":"payload"}"#.to_string(),
            }
        )
    }
}
