use std::error::Error;

#[derive(PartialEq, Debug, Clone)]
pub struct StoreError(String);

impl Error for StoreError {}

impl From<String> for StoreError {
    fn from(error: String) -> Self {
        StoreError(error)
    }
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ValidationError(String);

impl Error for ValidationError {}

impl From<String> for ValidationError {
    fn from(error: String) -> Self {
        ValidationError(error)
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
