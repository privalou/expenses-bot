use std::collections::HashMap;
use std::error::Error;

#[allow(dead_code)]
pub struct AppStore {
    data: HashMap<String, UserData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserData {
    user_id: String,
    current_dialog: DialogEntity,
    // todo: change to enum
    currency: Option<String>,
    history: History,
}

#[derive(Debug, Clone, PartialEq)]
struct History {
    data: HashMap<String, f32>,
}

impl Default for AppStore {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl AppStore {
    #[allow(dead_code)]
    pub fn new() -> AppStore {
        AppStore {
            data: HashMap::new(),
        }
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

    pub fn update_dialog(&mut self, patch: DialogPatch, id: &str) -> Option<&DialogEntity> {
        if let Some(user_data) = self.data.get_mut(id) {
            if let Some(command) = patch.command {
                user_data.current_dialog.command = command;
            }
            if let Some(step) = patch.step {
                user_data.current_dialog.step = step;
            }
            if let Some(data) = patch.data {
                user_data.current_dialog.data = data;
            }
            Some(&user_data.current_dialog)
        } else {
            None
        }
    }

    pub fn get_user_dialog(&self, id: &str) -> Option<&DialogEntity> {
        if let Some(user_data) = self.data.get(id) {
            Some(&user_data.current_dialog)
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

    fn delete(&mut self, id: &str) -> Option<()> {
        if self.data.remove(id).is_some() {
            Some(())
        } else {
            None
        }
    }
}

impl UserData {
    pub fn new(user_id: String) -> Result<UserData, ValidationError> {
        if user_id.is_empty() {
            return Err(ValidationError("User id can not be empty".to_string()));
        }

        Ok(UserData {
            user_id,
            current_dialog: DialogEntity::new(),
            currency: None,
            history: History::new(),
        })
    }
}

impl History {
    pub fn new() -> History {
        History {
            data: HashMap::new(),
        }
    }
}

/// todo: This struct is an emulation of the db entity. Should be reworked with types
#[derive(Debug, Clone, PartialEq)]
pub struct DialogEntity {
    pub command: String,
    pub step: String,
    pub data: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DialogPatch {
    pub command: Option<String>,
    pub step: Option<String>,
    pub data: Option<String>,
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
    pub fn step(&self) -> &String {
        &self.step
    }
    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn new() -> DialogEntity {
        DialogEntity {
            command: "/start".to_string(),
            step: "Step::FirstStep".to_string(),
            data: "{}".to_string(),
        }
    }

    pub fn new_with(
        command: String,
        step: String,
        data: String,
    ) -> Result<DialogEntity, ValidationError> {
        if command.is_empty() {
            return Err(ValidationError("Command can not be empty".to_string()));
        }

        Ok(DialogEntity {
            command,
            step,
            data,
        })
    }
}

impl DialogPatch {
    pub fn new_with(
        command: Option<String>,
        step: Option<String>,
        data: Option<String>,
    ) -> DialogPatch {
        DialogPatch {
            command,
            step,
            data,
        }
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
        let dialog = DialogEntity::new_with(
            "".to_string(),
            "Step::FirstStep".to_string(),
            "".to_string(),
        );
        assert!(dialog.is_err())
    }

    #[test]
    fn user_data_is_saved_at_store() {
        let mut store = AppStore::new();

        assert!(store.save_user("user_id").is_none());
        let retrieved_user_data = store
            .get_user_data("user_id")
            .expect("There is no such user");
        assert_eq!(retrieved_user_data.user_id, "user_id".to_string());
        assert!(retrieved_user_data.currency.is_none());
        assert_eq!(
            retrieved_user_data.current_dialog.command,
            "/start".to_string()
        );
        assert_eq!(
            retrieved_user_data.current_dialog.step,
            "Step::FirstStep".to_string()
        );
    }

    #[test]
    fn missing_user() {
        let store = AppStore::new();
        let user_id = Faker.fake::<String>();

        assert!(store.get_user_data(&user_id).is_none());
    }

    #[test]
    fn delete_works() {
        let mut store = AppStore::new();
        assert!(store.save_user("user_id").is_none());

        assert_eq!((), store.delete("user_id").unwrap());
        assert_eq!(store.get_user_dialog("user_id"), None);
    }

    #[test]
    fn updating_nothing_leaves_the_updatable_fields_unchanged() {
        let mut store = AppStore::new();
        assert_eq!(None, store.save_user("user_id"));

        let patch = DialogPatch {
            command: None,
            step: None,
            data: None,
        };

        let updated_dialog = store.update_dialog(patch, "user_id").unwrap();

        assert_eq!("Step::FirstStep", updated_dialog.step);
        assert_eq!("{}", updated_dialog.data);
        assert_eq!("/start", updated_dialog.command);
    }

    #[test]
    fn deleting_missing_user_returns_none() {
        let mut store = AppStore::new();
        let deleted_user = store.delete(&Faker.fake::<String>());
        assert_eq!(None, deleted_user)
    }
}
