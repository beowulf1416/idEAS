use crate::email::Email;
use uuid::Uuid;


#[derive(Debug)]
pub struct User {
    id: Uuid,
    active: bool,
    email: Email,
    permissions: Option<Vec<String>>
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
            email: email,
            permissions: None
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

    pub fn set_permissions(&mut self, permissions: Vec<String>) {
        self.permissions = Some(permissions);
    }

    pub fn has_permission(&self, permission: String) -> bool {
        if let Some(ps) = &self.permissions {
            return ps.contains(&permission);
        }
        return false;
    }
}