/**
 * 
 */
use log::{ info, error };

use std::env;
use std::str::FromStr;
use std::fs;

use actix_web::{ web };

use deadpool_postgres::{ Manager, ManagerConfig, Pool, RecyclingMethod };
use tokio_postgres::NoTls;
use tokio_postgres::config::{ Config };


pub fn configure(cfg: &mut web::ServiceConfig) {
    // database configuration
    if let Ok(url_db_file) = env::var("URL_DB_FILE") {
        info!("connection string file: {}", url_db_file);
        if let Ok(url_db) = fs::read_to_string(&url_db_file) {
            info!("connection string: {}", url_db);
            if url_db == "" {
                error!("connection string is empty from file");
            } else {
                match Config::from_str(&url_db) {
                    Ok(db_cfg) => {
                        let mgr = Manager::from_config(
                            db_cfg,
                            NoTls,
                            ManagerConfig {
                                recycling_method: RecyclingMethod::Fast
                            }
                        );
                        let pool = Pool::builder(mgr)
                            .max_size(16)
                            .build()
                            .unwrap();
            
                        info!("database pool created from file");
                        cfg.app_data(web::Data::new(pool.clone()));
                    }
                    Err(e) => {
                        error!("Unable to create config from file: '{}' {}", url_db_file, e);
                    }
                }
            }
        } else {
            error!("unable to read file: {}", url_db_file);
        }
    } else if let Ok(url_db) = env::var("URL_DB") {
        if url_db == "" {
            error!("connection string is empty");
        } else {
            match Config::from_str(&url_db) {
                Ok(db_cfg) => {
                    let mgr = Manager::from_config(
                        db_cfg,
                        NoTls,
                        ManagerConfig {
                            recycling_method: RecyclingMethod::Fast
                        }
                    );
                    let pool = Pool::builder(mgr)
                        .max_size(16)
                        .build()
                        .unwrap();
        
                    info!("database pool created");
                    cfg.app_data(web::Data::new(pool.clone()));
                }
                Err(e) => {
                    error!("Unable to create config: '{}' {}", url_db, e);
                }
            }
        }
    }
}