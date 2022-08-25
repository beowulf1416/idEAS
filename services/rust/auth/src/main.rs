extern crate log;

mod classes;
mod services;
mod endpoints;


use log::{ 
    info,
    debug,
    error
};

use serde::{
    Serialize,
    Deserialize
};

use actix_web::{ 
    HttpServer, 
    App, 
    web
};

use std::env;
use std::fs;

use crate::classes::app_config::ApplicationConfig;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting up");

    // let mut result = ();

    if let Ok(config) = get_config() {
        debug!("parsed config: {:?}", config);

        let bind_host = config.bind_host.clone();
        let bind_port = config.bind_port.clone();

        // let config_clone = config.clone();

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(config.clone()))

                // .configure(crate::services::config::configure)

                .service(web::scope("/status").configure(crate::endpoints::status::config))
        })
        .bind(format!("{}:{}", bind_host, bind_port))?
        .run();

        info!("Server is listening at https://{}:{}", bind_host, bind_port);
        return server.await;
    } else {
        error!("unable to retrieve configuration");
    }

    info!("Exiting...");
    return Ok(());
}

fn get_config() -> Result<ApplicationConfig, bool> {
    if let Ok(cfg) = env::var("CFG") {
        match fs::read_to_string(cfg) {
            Err(e) => {
                error!("unable to read contents of file: {:?}", e);
                return Err(false);
            }
            Ok(contents) => {
                let result: Result<ApplicationConfig, serde_json::Error>  = serde_json::from_str(&contents);
                match result {
                    Err(e) => {
                        error!("unable to parse contents: {:?}", e);
                        return Err(false);
                    }
                    Ok(config) => {
                        return Ok(config);
                    }
                }
            }
        }
    } else {
        error!("missing environment variable CFG");
        return Err(false);
    }
}