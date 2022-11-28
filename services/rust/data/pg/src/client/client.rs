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
        active: &bool,
        description: &str,
        address: &str,
        country_id: &i32,
        url: &str
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call client.client_add($1, $2, $3, $4, $5, $6, $7);",
            &[
                &client_id,
                &name,
                &active,
                &description,
                &address,
                &country_id,
                &url
            ]
        ).await;
    }

    pub async fn update(
        &self,
        client_id: &uuid::Uuid,
        name: &str,
        active: &bool,
        description: &str,
        address: &str,
        country_id: &i32,
        url: &str
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call client.client_update($1, $2, $3, $4, $5, $6, $7);",
            &[
                &client_id,
                &name,
                &active,
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

    pub async fn fetch(
        &self,
        filter: &str,
        active: &bool,
        items: &i32,
        page: &i32
    ) -> Result<Vec<common::client::Client>, DbError> {
        let sql = "select * from client.client_fetch($1, $2, $3, $4)";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                        &stmt,
                        &[
                            &filter,
                            &active,
                            &items,
                            &page
                        ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| common::client::Client {
                            id: r.get("id"),
                            active: r.get("active"),
                            name: r.get("name"),
                            description: r.get("description"),
                            address: r.get("address"),
                            country_id: r.get("country_id"),
                            url: r.get("url")
                        })
                        .collect();
                        return Ok(results);
                    }
                }
            }
        }
    }

    pub async fn get_default_client(
        &self
    ) -> Result<uuid::Uuid, DbError> {
        let sql = "select * from client.client_default_id()";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query_one(
                        &stmt,
                        &[
                        ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(r) => {
                        let result = r.get("client_default_id");
                        return Ok(result);
                    }
                }
            }
        }
    }

    pub async fn get(
        &self,
        client_id: &uuid::Uuid
    ) -> Result<common::client::Client, DbError> {
        let sql = "select * from client.client_get($1)";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query_one(
                        &stmt,
                        &[
                            &client_id
                        ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(r) => {
                        let result = common::client::Client {
                            id: r.get("id"),
                            active: r.get("active"),
                            name: r.get("name"),
                            description: r.get("description"),
                            address: r.get("address"),
                            country_id: r.get("country_id"),
                            url: r.get("url")
                        };
                        return Ok(result);
                    }
                }
            }
        }
    }

    pub async fn members(
        &self,
        client_id: &uuid::Uuid,
        active: &bool
    ) -> Result<Vec<common::iam::user::User>, DbError> {
        let sql = "select * from iam.user_client_members_fetch($1,$2)";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                        &stmt,
                        &[
                            &client_id,
                            &active
                        ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| common::iam::user::User {
                            id: r.get("id"),
                            active: r.get("active"),
                            email: r.get("email")
                        })
                        .collect();
                        return Ok(results);
                    }
                }
            }
        }
    }

    pub async fn member_invite(
        &self,
        client_id: &uuid::Uuid,
        email: &str
    ) -> Result<(), DbError> {
        let t_email = crate::types::email::Email::new(email);
        return self.0.call_sp(
            "call iam.user_client_invite($1, $2);",
            &[
                &client_id,
                &t_email
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
    async fn test_client_update() {
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
                            match clients.update(
                                &client_id,
                                &client_name,
                                &description,
                                &address,
                                &country_id,
                                &url
                            ).await {
                                Err(e) => {
                                    error!("unable to update client: {:?}", e);
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

    #[actix_rt::test]
    async fn test_client_fetch() {
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
                            match clients.fetch(
                                &"%",
                                &false,
                                &10,
                                &0
                            ).await {
                                Err(e) => {
                                    error!("unable to fetch clients: {:?}", e);
                                    assert!(false);
                                }
                                Ok(clients) => {
                                    debug!("clients: {:?}", clients);
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
