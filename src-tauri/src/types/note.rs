use serde::{Deserialize, Serialize};
use sqlx::FromRow;
/*


CREATE TABLE IF NOT EXISTS note (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
    relative_id      INTEGER NOT NULL,
    text             TEXT NOT NULL,
    pinned           BOOLEAN DEFAULT 0,
    FOREIGN KEY      (relative_id) REFERENCES relative(id) ON DELETE CASCADE
);
*/

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: u8,
    pub text: String,
    pub pinned: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreateNoteParams {
    pub relative_id: u32,
    pub text: String,
    pub pinned: bool,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateNoteParams {
    pub id: u8,
    pub text: String,
    pub pinned: bool,
}
