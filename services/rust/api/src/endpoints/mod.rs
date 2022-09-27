pub mod status;
pub mod countries;
pub mod clients;
pub mod auth;
pub mod user;


use serde::{
    Serialize,
    Deserialize
};
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    success: bool,
    message: String,
    data: Option<Value>
}


impl ApiResponse {

    pub fn new(
        success: bool,
        message: String,
        data: Option<Value>
    ) -> Self {
        return Self {
            success: success,
            message: message,
            data: data
        };
    }
}