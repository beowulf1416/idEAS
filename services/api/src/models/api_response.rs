use serde::{ Serialize, Deserialize };

use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
    pub data: Option<Value>
}