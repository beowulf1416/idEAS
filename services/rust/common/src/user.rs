#[derive(Debug)]
pub struct User {
    id: uuid::Uuid,
    active: bool,
    email: String
}

impl User {

    pub fn new(
        id: uuid::Uuid,
        active: bool,
        email: String
    ) -> Self {
        return Self {
            id: id,
            active: active,
            email: email
        };
    }
}