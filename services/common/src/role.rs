use serde::{ Serialize, Deserialize };
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct Role {
    id: Uuid,
    active: bool,
    name: String,
    description: String
}


impl Role {

    pub fn new(
        id: Uuid,
        active: bool,
        name: String,
        description: String
    ) -> Self {
        return Self {
            id: id,
            active: active,
            name: name,
            description: description
        };
    }

    pub fn id(&self) -> Uuid {
        return self.id;
    }

    pub fn active(&self) -> bool {
        return self.active;
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn description(&self) -> String {
        return self.description.clone();
    }
}