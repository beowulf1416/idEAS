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
}


#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use crate::Db;

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

                    let user_id = uuid::Uuid::new_v4();

                    let mut rng = rand::thread_rng();
                    let suffix: u8 = rng.gen();

                    let email = format!("email_{}@test.com", suffix);

                    match auth.register(
                        &user_id,
                        &email
                    ).await {
                        Err(e) => {
                            error!("unable to register new user {:?}", e);
                            assert!(false);
                        }
                        Ok(_) => {
                            match db.get_client().await {
                                Err(e) => {
                                    error!("unable to retrieve client 2");
                                    assert!(false);
                                }
                                Ok(client_2) => {
                                    let user = User::new(client_2);
                                    let active = true;
                                    match user.set_active(
                                        &user_id,
                                        &active
                                    ).await {
                                        Err(e) => {
                                            error!("unable to set active user status");
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

                    let user_id = uuid::Uuid::new_v4();

                    let mut rng = rand::thread_rng();
                    let suffix: u8 = rng.gen();

                    let email = format!("email_{}@test.com", suffix);

                    match auth.register(
                        &user_id,
                        &email
                    ).await {
                        Err(e) => {
                            error!("unable to register new user {:?}", e);
                            assert!(false);
                        }
                        Ok(_) => {
                            match db.get_client().await {
                                Err(e) => {
                                    error!("unable to retrieve client 2");
                                    assert!(false);
                                }
                                Ok(client_2) => {
                                    let user = User::new(client_2);
                                    let pw = "new_password";
                                    match user.set_password(
                                        &user_id,
                                        &pw
                                    ).await {
                                        Err(e) => {
                                            error!("unable to set password");
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
