/// bus
use log::{ info, error };

use std::env;
use std::fs;

use actix_web::{ web };

use bus::publisher::Publisher;


pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("utils::bus::configure()");
    if let Ok(bus_servers_cfg) = env::var("BUS_SERVERS_CFG") {
        if let Ok(hosts) = fs::read_to_string(&bus_servers_cfg) {
            if hosts == "" {
                let publisher = Publisher::new(vec!(hosts));
                cfg.app_data(web::Data::new(publisher));
            } else {
                error!("empty hosts configuration");
            }
        } else {
            error!("no hosts found");
        }
    } else {
        error!("unable to retrieve configuration file for bus");
    }
}