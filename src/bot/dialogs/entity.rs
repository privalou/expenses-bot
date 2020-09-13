#[derive(Debug, Clone, PartialEq)]
pub struct DialogEntity {
    pub user_id: String,
    pub command: String,
    pub step: String,
    pub data: String,
}
