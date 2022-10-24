use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub alpha_2: String,
    pub alpha_3: String
}

