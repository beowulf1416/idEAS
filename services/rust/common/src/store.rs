use serde::{
    Serialize,
    Deserialize
};

/// data store trait
pub trait Store {
    /// put data into store
    fn put(&self, key: &str, context: &serde_json::Value);

    /// retrieve data from store
    fn get(&self, key: &str, context: &serde_json::Value) -> Result<Vec, Error>;
}