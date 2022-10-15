pub mod status;
pub mod common;
// pub mod countries;
pub mod clients;
pub mod auth;
pub mod user;

pub mod admin;
pub mod inventory;

use log::{
    info
};


use serde::{
    Serialize,
    Deserialize
};
use serde_json::Value;

use actix_web::{
    HttpResponse, 
    Responder
};


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    success: bool,
    message: String,
    data: Option<Value>
    // data: Option<String>
}


impl ApiResponse {

    pub fn new(
        success: bool,
        message: String,
        // data: Option<String>
        data: Option<Value>
    ) -> Self {
        return Self {
            success: success,
            message: message,
            data: data
        };
    }
}


pub async fn default_options() -> impl Responder {
    info!("endpoints::common::default_options()");
    return HttpResponse::Ok()
        .finish();
}