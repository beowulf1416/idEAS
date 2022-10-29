use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct People {
    pub id: uuid::Uuid,
    pub given_name: String,
    pub middle_name: String,
    pub family_name: String,
    pub prefix: String,
    pub suffix: String
}