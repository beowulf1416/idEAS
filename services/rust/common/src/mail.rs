use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mail {
    pub to: String,
    pub subject: String,
    pub body: String
}