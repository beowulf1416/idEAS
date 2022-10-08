use serde::{
    Serialize,
    Deserialize
};


#[derive(Serialize, Deserialize)]
pub struct Registration {
    pub id: uuid::Uuid,
    pub email: String
}