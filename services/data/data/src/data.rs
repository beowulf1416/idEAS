use log::{ info, error };

use std::str::FromStr;

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