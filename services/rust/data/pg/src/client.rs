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

use crate::{
    DbError,
    Dbo
};


pub struct Client(Dbo);


impl Client {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
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
        return self.0.call_sp(
            "call client.client_add($1, $2, $3, $4, $5, $6);",
            &[
                &client_id,
                &name,
                &description,
                &address,
                &country_id,
                &url
            ]
        ).await;
    }

    pub async fn set_active(
        &self,
        client_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call client.client_set_active($1, $2);",
            &[
                &client_id,
                &active
            ]
        ).await;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use crate::Db;

    #[actix_rt::test] 
    async fn test_client_add() {
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
                    let country_id: i32 = 608;
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

    #[actix_rt::test]
    async fn test_client_set_active() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let clients = Client::new(client);

                    let client_id = uuid::Uuid::new_v4();

                    let mut rng = rand::thread_rng();
                    let suffix: u8 = rng.gen();
                    
                    let client_name = format!("client_name_{}", suffix);
                    let description = format!("description_{}", suffix);
                    let address = format!("address_{}", suffix);
                    let country_id: i32 = 608;
                    let url = format!("http://www.test-{}.com", suffix);

                    match clients.add(
                        &client_id,
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
                            let active = true;
                            match clients.set_active(
                                &client_id,
                                &true
                            ).await {
                                Err(e) => {
                                    error!("unable to set client active status: {:?}", e);
                                    assert!(false);
                                }
                                Ok(_) => {
                                    assert!(true);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            assert!(false);
        }
    }
}
