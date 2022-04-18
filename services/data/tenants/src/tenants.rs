/// tenant data object
use log::{ info, error, debug };

use actix_web::{
    web,
    HttpRequest
};

use deadpool_postgres::{ Manager, Pool };
use deadpool::managed::Object;

use uuid::Uuid;


pub struct Tenants {
    client: Object<Manager>
}


impl Tenants {

    pub fn new(
        client: Object<Manager>
    ) -> Self {
        return Tenants {
            client: client
        };
    }

    /// add tenant record
    pub async fn add(
        &self,
        id: Uuid,
        name: String
    ) -> Result<(), String> {
        info!("tenants::tenants::Tenants::add()");

        let result_stmt = self.client.prepare_cached(
            "call tenants.tenant_add($1, $2);"
        ).await;
        match result_stmt {
            Ok(stmt) => {
                if let Err(e) = self.client.query(
                    &stmt,
                    &[
                        &id,
                        &name
                    ]
                ).await {
                    error!("unable to add tenant: {:?}", e);

                    return Err(String::from("unable to add tenant"));
                } else {
                    return Ok(());
                }
            }
            Err(e) => {
                error!("unable to prepare statement to add tenant: {:?}", e);

                return Err(String::from("unable to prepare statement to add tenant"));
            }
        }
    }

    /// toggle tenant active status
    pub async fn active(
        &self,
        id: Uuid, 
        active: bool
    ) -> Result<(), String> {
        info!("tenants::tenants::Tenants::active()");

        let result_stmt = self.client.prepare_cached(
            "call tenants.tenant_set_active($1, $2);"
        ).await;

        match result_stmt {
            Ok(stmt) => {
                if let Err(e) = self.client.query(
                    &stmt,
                    &[
                        &id,
                        &active
                    ]
                ).await {
                    error!("unable to toggle tenant active status: {:?}", e);

                    return Err(String::from("unable to toggle tenant active status"));
                } else {
                    return Ok(());
                }
            }
            Err(e) => {
                error!("unable to prepare statement to toggle tenant active status: {:?}", e);

                return Err(String::from("unable to prepare statement to toggle tenant active status"));
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use log::{ info, error };

    use std::env;
    use std::str::FromStr;
    use uuid::Uuid;
    use deadpool_postgres::{ Manager, ManagerConfig, Pool, RecyclingMethod };
    use tokio_postgres::NoTls;
    use tokio_postgres::config::{ Config };

    use rand::Rng;


    use std::sync::Once;
    static INIT: Once = Once::new();


    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }


    #[actix_rt::test]
    async fn test_add() {
        initialize();

        if let Ok(url_db) = env::var("URL_DB") {
            info!("connection string: {}", url_db);
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

                    let mut rng = rand::thread_rng();
                    let suffix: u8 = rng.gen();
                    
                    let id = Uuid::new_v4();
                    let name = format!("tenant_{}", suffix);

                    let tenants = crate::tenants::Tenants::new(pool.get().await.unwrap());
                    if let Err(e) = tenants.add(id, name).await {
                        error!("error: {:?}", e);

                        assert!(false);
                    } else {
                        assert!(true);
                    }
                }
                Err(e) => {
                    error!("error: {:?}", e);

                    assert!(false);
                }
            }
        } else {
            assert!(false);
        }
    }

    #[actix_rt::test]
    async fn test_active() {
        initialize();

        if let Ok(url_db) = env::var("URL_DB") {
            info!("connection string: {}", url_db);
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

                    let mut rng = rand::thread_rng();
                    let suffix: u8 = rng.gen();
                    
                    let id = Uuid::new_v4();
                    let name = format!("tenant_{}", suffix);

                    let tenants = crate::tenants::Tenants::new(pool.get().await.unwrap());
                    if let Err(e) = tenants.add(id, name).await {
                        error!("error: {:?}", e);

                        assert!(false);
                    } else {
                        if let Err(e1) = tenants.active(id, true).await {
                            error!("unable to toggle tenant active status: {:?}", e1);

                            assert!(false);
                        } else {
                            assert!(true);
                        }
                    }
                }
                Err(e) => {
                    error!("error: {:?}", e);

                    assert!(false);
                }
            }
        } else {
            assert!(false);
        }
    }
}