use serde::{self, Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct State {
    pub pool: Arc<SqlitePool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelativeIndividual {
    pub id: u16,
    pub name: String,
    pub age: Option<u8>,
    pub sameness: Option<f32>,
    pub mother: Option<String>,
    pub father: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub pinned: bool,
    pub lost_reason: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemaleIndividual {
    pub id: u16,
    pub name: String,
    pub age: Option<u8>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeIndividual {
    pub id: u16,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRelativeParams {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub birthday: Option<String>,
    pub sameness: Option<f32>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemaleRelatives {
    names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MalesRelatives {
    names: Vec<String>,
}
