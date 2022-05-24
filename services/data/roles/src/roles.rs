/// roles data object
use log::{ info, error, debug };

use actix_web::{
    web,
    HttpRequest
};

use deadpool_postgres::{ Manager, Pool };
use deadpool::managed::Object;

use uuid::Uuid;
use data::data::Data;


pub struct Roles {
    client: Object<Manager>
}


impl Roles {

    pub fn new(
        client: Object<Manager>
    ) -> Self {
        return Roles {
            client: client
        };
    }


    pub async fn from_request(request: &HttpRequest) -> Result<Self, String> {
        if let Some(data) = request.app_data::<web::Data<Data>>() {
            if let Ok(client) = data.get_pool().get().await {   
                return Ok(Roles {
                    client: client
                });
            } else {
                error!("unable to retrieve database client");
                return Err(format!("unable to retrieve database client"));
            }
        } else {
            error!("unable to retrieve database pool");
            return Err(format!("unable to retrieve database pool"));
        }
    }

    /// add role
    pub async fn add(
        &self,
        role_id: &Uuid,
        tenant_id: &Uuid,
        name: &String,
        description: &String
    ) -> Result<(), String> {
        info!("roles::roles::Roles::add()");

        let result_stmt = self.client.prepare_cached(
            "call iam.role_add($1, $2, $3, $4);"
        ).await;
        match result_stmt {
            Ok(stmt) => {
                if let Err(e) = self.client.query(
                    &stmt,
                    &[
                        &role_id,
                        &tenant_id,
                        &name,
                        &description
                    ]
                ).await {
                    error!("unable to add role: {:?}", e);

                    return Err(String::from("unable to add role"));
                } else {
                    return Ok(());
                }
            }
            Err(e) => {
                error!("unable to prepare statement to add role: {:?}", e);

                return Err(String::from("unable to prepare statement to add role"));
            }
        }
    }


    /// toggle role active status
    pub async fn active(
        &self,
        id: Uuid,
        active: bool
    ) -> Result<(), String> {
        info!("roles::roles::Roles::active()");

        let result_stmt = self.client.prepare_cached(
            "call iam.role_set_active($1, $2);"
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
                    error!("unable to toggle role active status: {:?}", e);

                    return Err(String::from("unable to toggle role active status"));
                } else {
                    return Ok(());
                }
            }
            Err(e) => {
                error!("unable to prepare statement to toggle role active status: {:?}", e);

                return Err(String::from("unable to prepare statement to toggle role active status"));
            }
        }
    }

    // assign user to role
    pub async fn assign_user(
        &self,
        role_id: &Uuid,
        user_id: &Uuid
    ) -> Result<(), String> {
        info!("roles::roles::Roles::assign_user()");

        let result_stmt = self.client.prepare_cached(
            "call iam.assign_user_to_role($1, $2);"
        ).await;

        match result_stmt {
            Ok(stmt) => {
                if let Err(e) = self.client.query(
                    &stmt,
                    &[
                        &user_id,
                        &role_id
                    ]
                ).await {
                    error!("unable to assign user to role: {:?}", e);

                    return Err(String::from("unable to assign user to role"));
                } else {
                    return Ok(());
                }
            }
            Err(e) => {
                error!("unable to prepare statement to assign user to role: {:?}", e);

                return Err(String::from("unable to prepare statement to assign user to role"));
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

    use tenants::tenants::Tenants;


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
                    let name = format!("role_{}", suffix);
                    let desc = name.clone();

                    let tenants = Tenants::new(pool.get().await.unwrap());
                    if let Ok(tenant_id) = tenants.default_tenant_id().await {
                        let roles = crate::roles::Roles::new(pool.get().await.unwrap());
                        if let Err(e) = roles.add(id, tenant_id, name, desc).await {
                            error!("error: {:?}", e);
    
                            assert!(false);
                        } else {
                            assert!(true);
                        }
                    } else {
                        error!("unable to retrieve default tenant id");
                        assert!(false);
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
                    let desc = name.clone();

                    let tenants = Tenants::new(pool.get().await.unwrap());
                    if let Ok(tenant_id) = tenants.default_tenant_id().await {
                        let roles = crate::roles::Roles::new(pool.get().await.unwrap());
                        if let Err(e) = roles.add(id, tenant_id, name, desc).await {
                            error!("error: {:?}", e);
                            assert!(false);
                        } else {
                            if let Err(e) = roles.active(id, true).await {
                                error!("error: {:?}", e);
                                assert!(false);
                            } else {
                                assert!(true);
                            }
                        }
                    } else {
                        error!("unable to retrieve default tenant id");
                        assert!(false);
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

