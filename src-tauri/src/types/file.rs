use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: u8,
    pub file_name: String,
    pub file_path: String,
    pub size: u32,
    pub r#type: String,
    pub pinned: bool,
    pub created_at: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFileParams {
    pub relative_id: u8,
    pub file_path: String,
}
