use log::{
    info,
    error,
    debug
};


pub struct User {
    id: Option<uuid::Uuid>,
    email: String
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

    pub fn is_authenticated(&self) -> bool {
        return self.id != None;
    }
}