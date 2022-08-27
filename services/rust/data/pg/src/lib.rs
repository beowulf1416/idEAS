use log::{
    info,
    error,
    debug
};

use std::str::FromStr;

use deadpool_postgres::{ 
    Manager, 
    ManagerConfig, 
    Pool, 
    RecyclingMethod 
};
use tokio_postgres::NoTls;
use tokio_postgres::config::{ Config };

use config::{
    ApplicationConfig,
    ProviderType
};


#[derive(Debug, PartialEq)]
pub enum DbError {
    ClientError,
    DuplicateKeyError
}


pub struct Db {
    pool: Option<Pool>
}


impl Db {

    pub fn new(cfg: &ApplicationConfig) -> Self {
        for p in cfg.providers.clone() {
            if matches!(p.provider_type, ProviderType::Postgres) {
                debug!("provider: {:?}", p);
                for url in p.url {
                    match Config::from_str(&url) {
                        Err(e) => {
                            error!("unable to parse database connection string '{}' {:?}", url, e);
                        }
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
                                    Err(e) => {
                                        error!("unable to create database pool: {:?}", e);
                                    }
                                    Ok(pool) => {
                                        return Self {
                                            pool: Some(pool)
                                        };
                                    }
                                }
                        }
                    }
                }
            }
        }

        return Self {
            pool: None
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}
