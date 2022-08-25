use log::{
    info,
    debug,
    error
};

use std::env;
use std::fs;

use actix_web::{ web };

use crate::classes::app_config::ApplicationConfig;


pub fn configure(cfg: &mut web::ServiceConfig) {
    if let Ok(env_cfg) = env::var("CFG") {
        match fs::read_to_string(env_cfg) {
            Err(e) => {
                error!("unable to read contents of file: {:?}", e);
            }
            Ok(contents) => {
                let result: Result<ApplicationConfig, serde_json::Error>  = serde_json::from_str(&contents);
                match result {
                    Err(e) => {
                        error!("unable to parse contents: {:?}", e);
                    }
                    Ok(config) => {
                        cfg.app_data(web::Data::new(config));
                    }
                }
            }
        }
    } else {
        error!("missing environment variable CFG");
    }
}