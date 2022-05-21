/// tenant data object
use log::{ info, error, debug };

use actix_web::{
    web,
    HttpRequest
};

use deadpool_postgres::{ Manager, Pool };
use deadpool::managed::Object;

use uuid::Uuid;
use data::data::Data;

use common::tenant::Tenant;


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

    pub async fn from_request(request: &HttpRequest) -> Result<Self, String> {
        debug!("tenants::tenants::Tenants::from_request()");

        if let Some(data) = request.app_data::<web::Data<Data>>() {
            if let Ok(client) = data.get_pool().get().await {
                return Ok(Tenants {
                    client: client
                });
            } else {
                error!("unable to retrieve database client");

                return Err(String::from("unable to retrieve database client"));
            }
        } else {
            error!("unable to retrieve database pool");

            return Err(String::from("unable to retrieve database pool"));
        }
    }

    /// add tenant record
    pub async fn add(
        &self,
        id: &Uuid,
        name: &String
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
        id: &Uuid, 
        active: &bool
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

    /// get default tenant id
    pub async fn default_tenant_id(&self) -> Result<Uuid, String> {
        info!("tenants::tenants::Tenants::default_tenant_id()");

        let result_stmt = self.client.prepare_cached(
            "select tenants.tenant_default_id();"
        ).await;

        match result_stmt {
            Ok(stmt) => {
                match self.client.query_one(&stmt, &[]).await {
                    Ok(row) => {
                        let tenant_id: Uuid = row.get("tenant_default_id");

                        return Ok(tenant_id);
                    }
                    Err(e) => {
                        error!("unable to retrieve default tenant id: {:?}", e);
                        return Err(String::from("unable to retrieve default tenant id"));
                    }
                }
            }
            Err(e) => {
                error!("unable to prepare statement to toggle tenant active status: {:?}", e);

                return Err(String::from("unable to prepare statement to toggle tenant active status"));
            }
        }
    }

    /// get tenant info
    pub async fn get_tenant(&self, tenant_id: &Uuid) -> Result<Tenant, String> {
        info!("tenants::tenants::Tenants::get_tenant()");

        let result_stmt = self.client.prepare_cached(
            "select * from tenants.tenant_get_by_id($1)"
        ).await;

        match result_stmt {
            Ok(stmt) => {
                match self.client.query_one(&stmt, &[ &tenant_id]).await {
                    Ok(row) => {
                        let tenant_id: Uuid = row.get("id");
                        let active: bool = row.get("active");
                        let name: String = row.get("name");

                        return Ok(Tenant::new(
                            tenant_id,
                            active,
                            name
                        ));
                    }
                    Err(e) => {
                        error!("unable to retrieve tenant by id: {:?}", e);
                        return Err(format!("unable to retrieve tenant by id: {}", e));
                    }
                }
            }
            Err(e) => {
                error!("unable to prepare statement to retrieve tenant by id: {:?}", e);
                return Err(format!("unable to prepare statement to retrieve tenant by id"));
            }
        }
    }

    /// retrieve tenants
    pub async fn get_tenants(&self, filter: &String, items: &i64, page: &i64) -> Result<Vec<Tenant>, String> {
        info!("tenants::tenants::Tenants::get_tenants()");

        let result_stmt = self.client.prepare_cached(
            "select * from tenants.tenants_get($1, $1, $3)"
        ).await;

        match result_stmt {
            Ok(stmt) => {
                match self.client.query(&stmt, &[
                    &filter,
                    &items,
                    &page
                ]).await {
                    Ok(rows) => {
                        let mut tenants: Vec<Tenant> = Vec::new();

                        for r in rows {
                            let tenant_id: Uuid = r.get("id");
                            let active: bool = r.get("active");
                            let name: String = r.get("name");

                            tenants.push(Tenant::new(tenant_id, active, name));
                        }
                        
                        return Ok(tenants);
                    }
                    Err(e) => {
                        error!("unable to retrieve tenants: {:?}", e);
                        return Err(format!("unable to retrieve tenants: {}", e));
                    }
                }
            }
            Err(e) => {
                error!("unable to prepare statement to retrieve tenants: {:?}", e);
                return Err(format!("unable to prepare statement to retrieve tenants"));
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
                        if let Err(e) = tenants.active(id, true).await {
                            error!("error: {:?}", e);
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

    #[actix_rt::test]
    async fn test_default_tenant_id() {
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

                    let tenants = crate::tenants::Tenants::new(pool.get().await.unwrap());
                    if let Err(e) = tenants.default_tenant_id().await {
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
    async fn test_get_tenant() {
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

                    let tenants = crate::tenants::Tenants::new(pool.get().await.unwrap());
                    if let Ok(tenant_id) = tenants.default_tenant_id().await {
                        if let Err(e) = tenants.get_tenant(&tenant_id).await {
                            error!("unable to retrieve tenant info: {:?}", e);
                            assert!(false);
                        } else {
                            assert!(true);
                        }
                    } else {
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