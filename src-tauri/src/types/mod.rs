use serde::{self, Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};
use std::sync::Arc;

pub mod file;
pub mod note;

pub struct State {
    pub pool: Arc<SqlitePool>,
    pub files_dir: String,
    pub images_dir: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelativeIndividual {
    pub id: u32,
    #[sqlx(rename = "fname")]
    pub first_name: String,
    #[sqlx(rename = "mname")]
    pub middle_name: Option<String>,
    #[sqlx(rename = "lname")]
    pub last_name: String,
    pub birthday: Option<String>,
    pub age: Option<u32>,
    pub sameness: Option<f32>,
    pub mother_id: Option<u32>,
    pub father_id: Option<u32>,
    pub mother: Option<String>,
    pub father: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sex: String,
    pub pinned: bool,
    pub lost_reason: Option<String>,
    pub swarthy: Option<u8>,
    pub hotness: Option<u8>,
    pub crazy: Option<u8>,
    pub employable: Option<u8>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemaleIndividual {
    pub id: u32,
    pub name: String,
    pub age: Option<u32>,
    pub sameness: Option<u8>,
    pub swarthy: Option<u8>,
    pub hotness: Option<u8>,
    pub crazy: Option<u8>,
    pub mother: Option<String>,
    pub father: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub pinned: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeIndividual {
    pub id: u32,
    pub name: String,
    pub age: Option<u32>,
    pub sameness: Option<f32>,
    pub employable: Option<u8>,
    pub mother: Option<String>,
    pub father: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub pinned: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRelativeParams {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub birthday: Option<String>,
    pub died_at: Option<String>,
    pub sameness: Option<f32>,
    pub mother_id: Option<u32>,
    pub father_id: Option<u32>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sex: String,
    pub pinned: bool,
    pub lost_reason: Option<String>,
    pub swarthy: Option<u8>,
    pub hotness: Option<u8>,
    pub crazy: Option<u8>,
    pub employable: Option<u8>,
    pub address: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRelativeParams {
    pub id: u32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub birthday: Option<String>,
    pub died_at: Option<String>,
    pub sameness: Option<f32>,
    pub mother_id: Option<u32>,
    pub father_id: Option<u32>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sex: String,
    pub pinned: bool,
    pub lost_reason: Option<String>,
    pub swarthy: Option<u8>,
    pub hotness: Option<u8>,
    pub crazy: Option<u8>,
    pub employable: Option<u8>,
    pub address: Option<String>,
    pub state: Option<String>,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemaleRelatives {
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MalesRelatives {
    pub names: Vec<String>,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParantRelative {
    pub id: u32,
    #[sqlx(rename = "fname")]
    pub first_name: String,
    #[sqlx(rename = "mname")]
    pub middle_name: Option<String>,
    #[sqlx(rename = "lname")]
    pub last_name: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateImageParams {
    pub relative_id: u32,
    pub image_path: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: u16,
    pub filename: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedRelative {
    pub id: u32,
}
