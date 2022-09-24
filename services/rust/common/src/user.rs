#[derive(Debug, Clone)]
pub struct User {
    id: Option<uuid::Uuid>,
    active: bool,
    email: String
}

impl User {

    pub fn new(
        id: Option<uuid::Uuid>,
        active: bool,
        email: String
    ) -> Self {
        return Self {
            id: id,
            active: active,
            email: email
        };
    }

    pub fn is_authenticated(
        &self
    ) -> bool {
        return self.id.is_none();
    }

    pub fn id(&self) -> Option<uuid::Uuid> {
        return self.id;
    }

    pub fn active(&self) -> bool {
        return self.active;
    }

    pub fn email(&self) -> String {
        return format!("{}", self.email);
    }
}