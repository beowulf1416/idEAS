pub mod status;

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