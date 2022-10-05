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


pub struct Auth(Dbo);


impl Auth {

    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }


    pub async fn register(
        &self,
        user_id: &uuid::Uuid,
        token: &str,
        email: &str
    ) -> Result<(), DbError> {
        let t_email = crate::types::email::Email::new(email);
        return self.0.call_sp(
            "call iam.user_register($1, $2, $3);",
            &[
                &user_id,
                &token,
                &t_email
            ]
        ).await;
    }

    pub async fn authenticate(
        &self,
        email: &str,
        password: &str
    ) -> Result<bool, DbError> {
        let sql = "select * from iam.user_authenticate($1, $2);";
        match self.0.get_client().prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                let t_email = crate::types::email::Email::new(email);

                match self.0.get_client().query_one(
                    &stmt,
                    &[
                        &t_email,
                        &password
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to execute sql: {} {:?}", sql, e);
                        return Err(DbError::ClientError);
                    }
                    Ok(r) => {
                        let authentic: bool = r.get("user_authenticate");
                        return Ok(authentic);
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

    use crate::user::User;


    #[actix_rt::test] 
    async fn test_register() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let auth = Auth::new(client);

                    let new_id = uuid::Uuid::new_v4();

                    let mut rng = rand::thread_rng();
                    let suffix: u8 = rng.gen();

                    let token = format!("token_{}", suffix);
                    let email = format!("email_{}@test.com", suffix);

                    match auth.register(
                        &new_id,
                        &token,
                        &email
                    ).await {
                        Err(e) => {
                            error!("unable to register new user {:?}", e);
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
    async fn test_authenticate() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let auth = Auth::new(client);

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
                                    error!("unable to retrieve client 2 {:?}", e);
                                    assert!(false);
                                }
                                Ok(client_2) => {
                                    let password = format!("password_{}", suffix);
                                    let user = User::new(client_2);
                                    match user.set_password(
                                        &user_id,
                                        &password
                                    ).await {
                                        Err(e) => {
                                            error!("unable to set password {:?}", e);
                                            assert!(false);
                                        }
                                        Ok(_) => {
                                            let active = true;
                                            match user.set_active(
                                                &user_id,
                                                &active
                                            ).await {
                                                Err(e) => {
                                                    error!("unable to set user active status {:?}", e);
                                                    assert!(false);
                                                }
                                                Ok(_) => {
                                                    match auth.authenticate(
                                                        &email,
                                                        &password
                                                    ).await {
                                                        Err(e) => {
                                                            error!("unable to authenticate {:?}", e);
                                                            assert!(false);
                                                        }
                                                        Ok(authentic) => {
                                                            debug!("authenticate result: {:?}", authentic);
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
                }
            }
        } else {
            assert!(false);
        }
    }
}