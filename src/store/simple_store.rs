use std::collections::HashMap;
use std::error::Error;

#[allow(dead_code)]
pub struct AppStore {
    data: HashMap<String, DialogEntity>,
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

    #[allow(dead_code)]
    pub fn update(&mut self, id: &str, patch: DialogPatch) -> Option<&DialogEntity> {
        if let Some(dialog) = self.data.get_mut(id) {
            if let Some(command) = patch.command {
                dialog.command = command;
            }
            if let Some(step) = patch.step {
                dialog.step = step;
            }
            if let Some(data) = patch.data {
                dialog.data = data;
            }
            Some(dialog)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn save(&mut self, user_dialog: DialogEntity, id: &str) -> Option<String> {
        match self.data.insert(id.to_string(), user_dialog) {
            Some(value) => Some(value.user_id),
            None => None,
        }
    }

    pub fn get(&self, id: &str) -> Option<&DialogEntity> {
        self.data.get(id)
    }

    pub fn delete(&mut self, id: &str) -> Option<()> {
        if self.data.remove(id).is_some() {
            Some(())
        } else {
            None
        }
    }
}

/// This struct is an emulation of the db entity
/// Should be reworked with types
#[derive(Debug, Clone, PartialEq)]
pub struct DialogEntity {
    pub user_id: String,
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

impl DialogEntity {
    pub fn user_id(&self) -> &String {
        &self.user_id
    }
    pub fn command(&self) -> &String {
        &self.command
    }
    pub fn step(&self) -> &String {
        &self.step
    }
    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn new(user_id: String) -> Result<DialogEntity, ValidationError> {
        if user_id.is_empty() {
            return Err(ValidationError("User id can not be empty".to_string()));
        }
        Ok(DialogEntity {
            user_id,
            command: "/start".to_string(),
            step: "Step::FirstStep".to_string(),
            data: "".to_string(),
        })
    }

    pub fn new_with(
        user_id: String,
        command: String,
        step: String,
        data: String,
    ) -> Result<DialogEntity, ValidationError> {
        if user_id.is_empty() {
            return Err(ValidationError("User id can not be empty".to_string()));
        }
        if command.is_empty() {
            return Err(ValidationError("Command can not be empty".to_string()));
        }

        Ok(DialogEntity {
            user_id,
            command,
            step,
            data,
        })
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
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn user_id_cannot_be_empty() {
        let dialog = DialogEntity::new_with(
            "".to_string(),
            "/start".to_string(),
            "Step::FirstStep".to_string(),
            "".to_string(),
        );
        assert!(dialog.is_err())
    }

    #[test]
    fn command_cannot_be_empty() {
        let dialog = DialogEntity::new_with(
            "foo".to_string(),
            "".to_string(),
            "Step::FirstStep".to_string(),
            "".to_string(),
        );
        assert!(dialog.is_err())
    }

    #[test]
    fn dialog_is_saved_at_store() {
        let dialog = generate_start_dialog("user_id".to_string());
        let mut store = AppStore::new();

        assert!(store.save(dialog.clone(), &dialog.user_id).is_none());
        let retrieved_ticket = store
            .get(&"user_id".to_string())
            .expect("There is no such user");
        assert_eq!(retrieved_ticket.user_id, dialog.user_id);
        assert_eq!(retrieved_ticket.command, dialog.command);
        assert_eq!(retrieved_ticket.data, dialog.data);
        assert_eq!(retrieved_ticket.step, dialog.step);
    }

    #[test]
    fn missing_user() {
        let store = AppStore::new();
        let user_id = Faker.fake::<String>();

        assert_eq!(store.get(&user_id), None);
    }

    #[test]
    fn delete_works() {
        let mut store = AppStore::new();
        let dialog = generate_start_dialog("user_id".to_string());
        store.save(dialog.clone(), "user_id");

        assert_eq!((), store.delete(&dialog.user_id).unwrap());
        assert_eq!(store.get(&dialog.user_id), None);
    }

    #[test]
    fn updating_nothing_leaves_the_updatable_fields_unchanged() {
        let mut store = AppStore::new();
        let start_dialog = generate_start_dialog("user_id".to_string());
        assert_eq!(None, store.save(start_dialog.clone(), "user_id"));

        let patch = DialogPatch {
            command: None,
            step: None,
            data: None,
        };

        let updated_dialog = store.update(&start_dialog.user_id, patch).unwrap();

        assert_eq!(start_dialog.user_id, updated_dialog.user_id);
        assert_eq!(start_dialog.step, updated_dialog.step);
        assert_eq!(start_dialog.data, updated_dialog.data);
        assert_eq!(start_dialog.command, updated_dialog.command);
    }

    #[test]
    fn deleting_missing_user_returns_none() {
        let mut store = AppStore::new();
        let deleted_user = store.delete(&Faker.fake::<String>());
        assert_eq!(None, deleted_user)
    }

    fn generate_start_dialog(user_id: String) -> DialogEntity {
        DialogEntity::new(user_id).expect("Wrong user id")
    }
}
