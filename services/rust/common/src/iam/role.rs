use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    pub id: uuid::Uuid,
    pub active: bool,
    pub name: String,
    pub description: String
}