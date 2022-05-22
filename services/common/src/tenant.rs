/// tenant class
use log::{ info, error, debug };

use serde::{ Serialize, Deserialize };
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct Tenant {
    id: Uuid,
    active: bool,
    name: String
}


impl Tenant {

    pub fn new(
        id: Uuid,
        active: bool,
        name: String
    ) -> Self {
        return Tenant {
            id: id,
            active: active,
            name: name
        };
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }

    pub fn get_active(&self) -> bool {
        return self.active;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}