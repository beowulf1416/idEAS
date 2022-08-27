use log::{
    info,
    error,
    debug
};


use actix_web::{ web };

use config::{
    ApplicationConfig,
    ProviderType
};



pub struct Data {

}


impl Data {

    pub fn new(cfg: &ApplicationConfig) -> Self {
        return Data {

        };
    }
}