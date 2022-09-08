use log::{
    info,
    error,
    debug
};


use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager
};
use tokio_postgres::{
    error::SqlState
};

use crate::DbError;


pub struct Client {
    client: Object<Manager>
}


impl Client {
    pub fn new(client: Object<Manager>) -> Self {
        return Self {
            client: client
        };
    }

    pub async fn add(
        &self,
        client_id: &uuid::Uuid,
        name: &str,
        description: &str,
        address: &str,
        country_id: &i32,
        url: &str
    ) -> Result<(), DbError> {
        let sql = "call client.client_add($1, $2, $3, $4, $5, $6);";
        match self.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.client.execute(
                    sql,
                    &[
                        &client_id,
                        &name,
                        &description,
                        &address,
                        &country_id,
                        &url
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to execute sql: {} {:?}", sql, e);
                        return Err(DbError::ClientError);
                    }
                    Ok(_rows) => {
                        return Ok(());
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::env;

    use std::sync::Once;
    static INIT: Once = Once::new();

    use super::*;

    use rand::Rng;
    use crate::Db;


    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }

    #[actix_rt::test] 
    async fn test_client_add() {
        // initialize();

        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let clients = Client::new(client);

                    let new_id = uuid::Uuid::new_v4();

                    let mut rng = rand::thread_rng();
                    let suffix: u8 = rng.gen();
                    
                    let client_name = format!("client_name_{}", suffix);
                    let description = format!("description_{}", suffix);
                    let address = format!("address_{}", suffix);
                    let country_id: i32 = 8;
                    let url = format!("http://www.test-{}.com", suffix);


                    match clients.add(
                        &new_id,
                        &client_name,
                        &description,
                        &address,
                        &country_id,
                        &url
                    ).await {
                        Err(e) => {
                            error!("unable to add client {:?}", e);
                            assert!(false);
                        }
                        Ok(_) => {
                            assert!(true);
                        }
                    }
                }
            }
        } else {
            assert!(false);
        }
    }
}
