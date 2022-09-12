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


pub struct Organization(Dbo);

impl Organization {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn add(
        &self,
        client_id: &uuid::Uuid,
        organization_id: &uuid::Uuid,
        name: &str,
        description: &str
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call client.organization_add($1, $2, $3, $4);",
            &[
                &client_id,
                &organization_id,
                &name,
                &description
            ]
        ).await;
    }


    pub async fn update(
        &self,
        client_id: &uuid::Uuid,
        organization_id: &uuid::Uuid,
        name: &str,
        description: &str
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call client.organization_update($1, $2, $3, $4);",
            &[
                &client_id,
                &organization_id,
                &name,
                &description
            ]
        ).await;
    }

    pub async fn set_active(
        &self,
        organization_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call client.organization_set_active($1, $2);",
            &[
                &organization_id,
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
    async fn test_organization_add() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let clients = crate::client::Client::new(client);

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
                            match db.get_client().await {
                                Err(e) => {
                                    error!("unable to retrieve client 2: {:?}", e);
                                    assert!(false);
                                }
                                Ok(client_2) => {
                                    let organization = Organization::new(client_2);

                                    let org_id = uuid::Uuid::new_v4();
                                    let org_name = format!("organization_name_{}", suffix);
                                    let org_desc = format!("organization_description_{}", description);

                                    match organization.add(
                                        &client_id,
                                        &org_id,
                                        &org_name,
                                        &org_desc
                                    ).await {
                                        Err(e) => {
                                            error!("unable to add organization {:?}", e);
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
                }
            }
        } else {
            assert!(false);
        }
    }


    #[actix_rt::test]
    async fn test_organization_update() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let clients = crate::client::Client::new(client);

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
                            match db.get_client().await {
                                Err(e) => {
                                    error!("unable to retrieve client 2: {:?}", e);
                                    assert!(false);
                                }
                                Ok(client_2) => {
                                    let organization = Organization::new(client_2);

                                    let org_id = uuid::Uuid::new_v4();
                                    let org_name = format!("organization_name_{}", suffix);
                                    let org_desc = format!("organization_description_{}", description);

                                    match organization.add(
                                        &client_id,
                                        &org_id,
                                        &org_name,
                                        &org_desc
                                    ).await {
                                        Err(e) => {
                                            error!("unable to add organization {:?}", e);
                                            assert!(false);
                                        }
                                        Ok(_) => {
                                            let org_name_2 = format!("organization_name_2_{}", suffix);
                                            match organization.update(
                                                &client_id,
                                                &org_id,
                                                &org_name_2,
                                                &org_desc
                                            ).await {
                                                Err(e) => {
                                                    error!("unable to update organization: {:?}", e);
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
                        }
                    }
                }
            }
        } else {
            assert!(false);
        }
    }


    #[actix_rt::test]
    async fn test_organization_set_active() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let clients = crate::client::Client::new(client);

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
                            match db.get_client().await {
                                Err(e) => {
                                    error!("unable to retrieve client 2: {:?}", e);
                                    assert!(false);
                                }
                                Ok(client_2) => {
                                    let organization = Organization::new(client_2);

                                    let org_id = uuid::Uuid::new_v4();
                                    let org_name = format!("organization_name_{}", suffix);
                                    let org_desc = format!("organization_description_{}", description);

                                    match organization.add(
                                        &client_id,
                                        &org_id,
                                        &org_name,
                                        &org_desc
                                    ).await {
                                        Err(e) => {
                                            error!("unable to add organization {:?}", e);
                                            assert!(false);
                                        }
                                        Ok(_) => {
                                            match organization.set_active(
                                                &org_id,
                                                &true
                                            ).await {
                                                Err(e) => {
                                                    error!("unable to set organization active status: {:?}", e);
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
                        }
                    }
                }
            }
        } else {
            assert!(false);
        }
    }
}