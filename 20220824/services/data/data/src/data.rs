use log::{ info, error };

use std::env;
use std::str::FromStr;
use std::fs;

use actix_web::{ web };

use deadpool_postgres::{ Manager, ManagerConfig, Pool, RecyclingMethod };
use tokio_postgres::NoTls;
use tokio_postgres::config::{ Config };


#[derive(Clone)]
pub struct Data {
    pool: Pool
}

impl Data {

    pub fn new(connection_string: String) -> Result<Self, String > {
        match Config::from_str(&connection_string) {
            Ok(db_cfg) => {
                let mgr = Manager::from_config(
                    db_cfg,
                    NoTls,
                    ManagerConfig {
                        recycling_method: RecyclingMethod::Fast
                    }
                );

                match Pool::builder(mgr)
                    .max_size(16)
                    .build() {
                        Ok(pool) => {
                            return Ok(Data {
                                pool: pool
                            });
                        }
                        Err(e) => {
                            error!("unable to create pool object: {:?}", e);
                            return Err(String::from("unable to create pool object"));
                        }
                }

                
            }
            Err(e) => {
                error!("unable to create config object: {:?}", e);
                return Err(String::from("unable to create config object"));
            }
        }
    }

    pub fn get_pool(&self) -> Pool {
        return self.pool.clone();
    }
}


pub fn configure(cfg: &mut web::ServiceConfig) {
    // database configuration
    if let Ok(url_db_file) = env::var("URL_DB_FILE") {
        info!("connection string file: {}", url_db_file);
        if let Ok(url_db) = fs::read_to_string(&url_db_file) {
            info!("connection string: {}", url_db);
            if url_db == "" {
                error!("connection string is empty from file");
            } else {
                if let Ok(data) = Data::new(url_db) {
                    cfg.app_data(web::Data::new(data.clone()));
                }
            }
        } else {
            error!("unable to read file: {}", url_db_file);
        }
    } else if let Ok(url_db) = env::var("URL_DB") {
        if url_db == "" {
            error!("connection string is empty");
        } else {
            if let Ok(data) = Data::new(url_db) {
                cfg.app_data(web::Data::new(data.clone()));
            }
        }
    }
}