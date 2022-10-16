use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: uuid::Uuid,
    pub active: bool,
    pub name: String,
    pub description: String,
    pub address: String,
    pub country_id: i32,
    pub url: String
}