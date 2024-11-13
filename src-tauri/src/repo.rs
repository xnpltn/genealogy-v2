use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relative {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Female {
    id: u16,
    name: String,
    age: Option<u8>,
    sameness: Option<u8>,
    swarthy: Option<u8>,
    hotness: Option<u8>,
    crazy: Option<u8>,
    mother: Option<String>,
    father: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    pinned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    id: u16,
    name: String,
    age: Option<u32>,
    sameness: Option<f32>,
    employable: Option<u8>,
    mother: Option<String>,
    father: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    pinned: bool,
}
