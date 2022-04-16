/// db
use log::{ info, error, debug };

use std::env;
use std::str::FromStr;
use std::fs;

use deadpool_postgres::{ Manager, ManagerConfig, Pool, RecyclingMethod };
use tokio_postgres::NoTls;
use tokio_postgres::config::{ Config };


pub fn configure() -> Result<Pool, String> {
    if let Ok(url_db_file) = env::var("URL_DB_FILE") {
        info!("connection string file: {}", url_db_file);
        if let Ok(url_db) = fs::read_to_string(&url_db_file) {
            info!("connection string: {}", url_db);
            if url_db == "" {
                error!("connection string is empty from file");
                
                return Err(String::from("configuration file is empty or incorret"));
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

                        return Ok(pool);
                    }
                    Err(e) => {
                        error!("Unable to create config from file: '{}' {}", url_db_file, e);

                        return Err(String::from("unable to create config file"));
                    }
                }
            }
        } else {
            error!("unable to read file: {}", url_db_file);

            return Err(String::from("unable to read config file"));
        }
    } else {
        error!("environment variable 'URL_DB_FILE' is empty");

        return Err(String::from("environment variable 'URL_DB_FILE' is empty"));
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_configure() {
        if let Ok(pool) = configure() {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}