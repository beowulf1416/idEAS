use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Permission {
    pub name: String
}