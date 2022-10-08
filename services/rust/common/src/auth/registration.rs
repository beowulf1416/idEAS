use serde::{
    Serialize,
    Deserialize
};


#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    pub id: uuid::Uuid,
    pub email: String
}