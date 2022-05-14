use serde::{ Serialize, Deserialize };

use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub enum ApiResponseStatus {
    Success,
    Error
}

// impl ApiResponseStatus {

//     pub fn as_str(&self) -> String {
//         match self {
//             ApiResponseStatus::Success => String::from("success"),
//             ApiResponseStatus::Error => String::from("error")
//         }
//     }
// }


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: ApiResponseStatus,
    pub message: String,
    pub data: Option<Value>
}