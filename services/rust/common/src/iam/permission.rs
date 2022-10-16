use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: i32,
    pub name: String
}