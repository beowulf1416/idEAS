use crate::email::Email;
use uuid::Uuid;

pub struct User {
    id: Uuid,
    active: bool,
    email: Email
}


impl User {

    pub fn new(
        id: Uuid,
        active: bool,
        email: Email
    ) -> Self {
        return User {
            id: id,
            active: active,
            email: email
        };
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }

    pub fn get_active(&self) -> bool {
        return self.active.clone();
    }

    pub fn get_email(&self) -> Email {
        return self.email.clone();
    }
}