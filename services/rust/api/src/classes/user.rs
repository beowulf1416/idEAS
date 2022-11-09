use log::{
    info,
    error,
    debug
};

use serde::{
    Serialize,
    Deserialize
};



pub struct User {
    pub id: uuid::Uuid,
    pub email: String
}


impl User {

    pub fn new(
        id: Option<uuid::Uuid>,
        email: String
    ) -> Self {
        return Self {
            id: id,
            email: email
        };
    }
}