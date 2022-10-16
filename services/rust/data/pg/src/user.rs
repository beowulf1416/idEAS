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

// use common::user::User;


pub struct User(Dbo);

impl User {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn set_active(
        &self,
        user_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call iam.user_set_active($1, $2);",
            &[
                &user_id,
                &active
            ]
        ).await;
    }

    pub async fn set_password(
        &self,
        user_id: &uuid::Uuid,
        password: &str
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call iam.user_set_password($1, $2);",
            &[
                &user_id,
                &password
            ]
        ).await;
    }


    pub async fn get_by_email(
        &self,
        email: &str
    ) -> Result<common::user::User, DbError> {
        let sql = "select * from iam.user_get_by_email($1);";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare sql: {:?}", e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                let t_email = crate::types::email::Email::new(email);
                match self.0.client.query_one(
                    &stmt,
                    &[
                        &t_email
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve user: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(r) => {
                        return Ok(common::user::User::new(
                            Some(r.get("id")),
                            r.get("active"),
                            r.get("email")
                        ));
                    }
                }
            }
        }
    }


    pub async fn fetch_clients(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<Vec<common::client::Client>, DbError> {
        let sql = "select * from iam.user_client_fetch($1, $2)";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                        &stmt,
                        &[
                            &user_id,
                            &true
                        ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| common::client::Client {
                            id: r.get("client_id"),
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

    pub async fn fetch_permissions(
        &self,
        user_id: &uuid::Uuid,
        client_id: &uuid::Uuid
    ) -> Result<Vec<common::iam::permission::Permission>, DbError> {
        let sql = "select * from iam.user_permissions_fetch($1, $2)";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                        &stmt,
                        &[
                            &user_id,
                            &client_id
                        ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| common::iam::permission::Permission {
                            name: r.get("name")
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
                        debug!("row: {:?}", r);
                        let result = r.get("client_default_id");
                        return Ok(result);
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;

    use rand::Rng;
    use crate::Db;

    lazy_static!{
        static ref t_user_id: uuid::Uuid = {
            return uuid::Uuid::new_v4();
        };
    }

    #[actix_rt::test] 
    async fn test_user_set_active() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let auth = crate::auth::Auth::new(client);

                    let mut rng = rand::thread_rng();
                    let suffix: u8 = rng.gen();

                    let email = format!("email_{}@test.com", suffix);
                    let pw = format!("ThisisaTest88**");

                    match auth.sign_up(
                        &t_user_id,
                        &email,
                        &pw
                    ).await {
                        Err(e) => {
                            error!("unable to sign up user");
                            assert!(false);
                        }
                        Ok(_) => {
                            match db.get_client().await {
                                Err(e) => {
                                    error!("unable to retrieve client");
                                    assert!(false);
                                }
                                Ok(client_2) => {
                                    let user_dbo = User::new(client_2);
                                    match user_dbo.set_active(
                                        &t_user_id,
                                        &true
                                    ).await {
                                        Err(e) => {
                                            error!("unable to set user active status");
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
    async fn test_user_set_password() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let auth = crate::auth::Auth::new(client);

                    let mut rng = rand::thread_rng();
                    let suffix: u8 = rng.gen();

                    let email = format!("email_{}@test.com", suffix);
                    let pw = format!("ThisisaTest88**");

                    match auth.sign_up(
                        &t_user_id,
                        &email,
                        &pw
                    ).await {
                        Err(e) => {
                            error!("unable to sign up user");
                            assert!(false);
                        }
                        Ok(_) => {
                            match db.get_client().await {
                                Err(e) => {
                                    error!("unable to retrieve client");
                                    assert!(false);
                                }
                                Ok(client_2) => {
                                    let user_dbo = User::new(client_2);
                                    match user_dbo.set_password(
                                        &t_user_id,
                                        &"NewPassword88**"
                                    ).await {
                                        Err(e) => {
                                            error!("unable to set user password");
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
}
