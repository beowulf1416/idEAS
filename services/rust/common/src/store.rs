use serde::{
    Serialize,
    Deserialize
};

pub trait Store {
    fn write(&self, key: &str, data: &serde_json::Value);

    fn get(&self, key: &str, data: &serde_json::Value) -> Result<Vec, Error>;
}