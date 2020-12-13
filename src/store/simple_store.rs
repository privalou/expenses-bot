use std::collections::HashMap;
use std::error::Error;

#[allow(dead_code)]
pub struct Store {
    data: HashMap<String, UserData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserData {
    pub user_id: String,
    pub current_dialog: Option<DialogEntity>,
    // todo: change to enum
    pub currency: Option<String>,
    pub history: History,
}

pub struct UserDataPatch {
    pub current_dialog: Option<DialogEntity>,
    pub currency: Option<String>,
    pub history: Option<History>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct History {
    data: HashMap<String, f32>,
}

/// todo: This struct is an emulation of the db entity. Should be reworked with types
#[derive(Debug, Clone, PartialEq)]
pub struct DialogEntity {
    pub command: String,
    pub step: Option<String>,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl Store {
    #[allow(dead_code)]
    pub fn new() -> Store {
        Store {
            data: HashMap::new(),
        }
    }

    pub fn is_registered(&self, id: &str) -> bool {
        self.data.contains_key(id)
    }

    pub fn save_user(&mut self, id: &str) -> Option<String> {
        match self.data.insert(
            id.to_string(),
            UserData::new(id.to_string()).expect("Invalid user id"),
        ) {
            Some(value) => Some(value.user_id),
            None => None,
        }
    }

    pub fn get_user_dialog(&self, id: &str) -> Option<&DialogEntity> {
        if let Some(user_data) = self.data.get(id) {
            if let Some(current_dialog) = &user_data.current_dialog {
                Some(&current_dialog)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_user_data(&self, id: &str) -> Option<&UserData> {
        if let Some(user_data) = self.data.get(id) {
            Some(&user_data)
        } else {
            None
        }
    }

    pub fn update_dialog(
        &mut self,
        dialog: Option<DialogEntity>,
        id: &str,
    ) -> Result<(), StoreError> {
        if let Some(user_data) = self.data.get_mut(id) {
            if let Some(dialog) = dialog {
                user_data.current_dialog = Some(DialogEntity {
                    command: dialog.command,
                    step: dialog.step,
                });
            } else {
                user_data.current_dialog = None;
            }
            Ok(())
        } else {
            Err(StoreError(
                "Impossible to update nonexisting value".to_string(),
            ))
        }
    }

    pub fn update_currency(&mut self, new_currency: &str, id: &str) -> Option<String> {
        if let Some(user_data) = self.data.get_mut(id) {
            user_data.currency = Some(new_currency.to_string());
            // todo: how to get rid off cloning for Option<String>
            let currency = user_data.currency.clone().expect("Currency haven't saved");
            Some(currency)
        } else {
            None
        }
    }

    pub fn update_user_data(
        &mut self,
        user_data_patch: UserDataPatch,
        id: &str,
    ) -> Result<(), StoreError> {
        if let Some(user_data) = self.data.get_mut(id) {
            if let Some(currency) = user_data_patch.currency {
                user_data.currency = Some(currency);
            }
            if let Some(current_dialog) = user_data_patch.current_dialog {
                user_data.current_dialog = Some(current_dialog);
            }
            if let Some(history) = user_data_patch.history {
                user_data.history = history;
            }
            Ok(())
        } else {
            Err(StoreError(
                "Impossible to update not existing user".to_string(),
            ))
        }
    }

    fn delete(&mut self, id: &str) -> Option<()> {
        if self.data.remove(id).is_some() {
            Some(())
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct StoreError(String);

impl Error for StoreError {}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UserData {
    pub fn new(user_id: String) -> Result<UserData, ValidationError> {
        if user_id.is_empty() {
            return Err(ValidationError("User id can not be empty".to_string()));
        }

        Ok(UserData {
            user_id,
            current_dialog: Some(
                DialogEntity::new_with("/start".to_string(), Some("CurrencySelection".to_string()))
                    .expect("Invalid command"),
            ),
            currency: None,
            history: History::new(),
        })
    }
}

impl Default for UserDataPatch {
    fn default() -> Self {
        Self::new()
    }
}

impl UserDataPatch {
    pub fn new() -> UserDataPatch {
        UserDataPatch {
            currency: None,
            current_dialog: None,
            history: None,
        }
    }

    pub fn new_with(
        current_dialog: Option<DialogEntity>,
        currency: Option<String>,
        history: Option<History>,
    ) -> UserDataPatch {
        UserDataPatch {
            current_dialog,
            currency,
            history,
        }
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    pub fn new() -> History {
        History {
            data: HashMap::new(),
        }
    }
}

impl Default for DialogEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogEntity {
    pub fn command(&self) -> &String {
        &self.command
    }
    pub fn step(&self) -> &Option<String> {
        &self.step
    }

    pub fn new() -> DialogEntity {
        DialogEntity {
            command: "/start".to_string(),
            step: None,
        }
    }

    pub fn new_with(
        command: String,
        step: Option<String>,
    ) -> Result<DialogEntity, ValidationError> {
        if command.is_empty() {
            return Err(ValidationError(
                "Can not create dialog patch with empty command.".to_string(),
            ));
        }
        Ok(DialogEntity { command, step })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ValidationError(String);

impl Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use super::*;

    #[test]
    fn user_id_cannot_be_empty() {
        let user_data = UserData::new("".to_string());
        assert!(user_data.is_err())
    }

    #[test]
    fn command_cannot_be_empty() {
        let dialog = DialogEntity::new_with("".to_string(), None);
        assert!(dialog.is_err())
    }

    #[test]
    fn is_registered_should_return_false_if_no_such_user_at_store() {
        let store = Store::new();
        assert!(!store.is_registered("user_id"));
    }

    #[test]
    fn is_registered_should_return_true_if_user_at_store() {
        let mut store = Store::new();
        assert!(store.save_user("user_id").is_none());
        assert!(store.is_registered("user_id"));
    }

    #[test]
    fn user_data_is_saved_at_store() {
        let mut store = Store::new();

        assert!(store.save_user("user_id").is_none());
        let retrieved_user_data = store
            .get_user_data("user_id")
            .expect("There is no such user");
        assert_eq!(retrieved_user_data.user_id, "user_id".to_string());
        assert!(retrieved_user_data.currency.is_none());
        let option_dialog = &retrieved_user_data.current_dialog;
        assert!(option_dialog.is_some());
        let retrieved_dialog = option_dialog.as_ref().unwrap();
        assert_eq!(retrieved_dialog.command, "/start".to_string());
        assert!(retrieved_dialog.step.is_some());
        assert_eq!(
            retrieved_dialog.step.as_ref().unwrap(),
            &"CurrencySelection"
        );
    }

    #[test]
    fn missing_user() {
        let store = Store::new();
        let user_id = Faker.fake::<String>();

        assert!(store.get_user_data(&user_id).is_none());
    }

    #[test]
    fn update_dialog_works() {
        let mut store = Store::new();
        assert_eq!(None, store.save_user("user_id"));

        let dialog =
            DialogEntity::new_with("/test".to_string(), None).expect("Invalid DialogEntity");

        assert!(store.update_dialog(Some(dialog), "user_id").is_ok());

        let updated_dialog_option = store.get_user_dialog("user_id");
        assert!(updated_dialog_option.is_some());
        let updated_dialog = updated_dialog_option.unwrap();
        assert_eq!("/test", updated_dialog.command);
        let step = &updated_dialog.step;
        assert!(step.is_none());
    }

    #[test]
    fn update_currency_works() {
        let mut store = Store::new();

        assert!(store.save_user("user_id").is_none());
        assert_eq!(store.update_currency("$", "user_id"), Some("$".to_string()));
        let user_data = store
            .get_user_data("user_id")
            .expect("No user data for such user");
        assert_eq!(user_data.currency, Some("$".to_string()));
    }

    #[test]
    fn delete_works() {
        let mut store = Store::new();
        assert!(store.save_user("user_id").is_none());

        assert_eq!((), store.delete("user_id").unwrap());
        assert_eq!(store.get_user_dialog("user_id"), None);
    }

    #[test]
    fn deleting_missing_user_returns_none() {
        let mut store = Store::new();
        let deleted_user = store.delete(&Faker.fake::<String>());
        assert_eq!(None, deleted_user)
    }
}
