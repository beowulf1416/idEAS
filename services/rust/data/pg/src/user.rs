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


pub struct User {
    client: Object<Manager>
}


impl User {

    pub fn new(client: Object<Manager>) -> Self {
        return Self {
            client: client
        };
    }

    pub async fn set_active(
        &self,
        user_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DbError> {
        let sql = "call iam.user_set_active($1, $2);";
        match self.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.client.execute(
                    sql,
                    &[
                        &user_id,
                        &active
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


    pub async fn set_password(
        &self,
        user_id: &uuid::Uuid,
        password: &str
    ) -> Result<(), DbError> {
        let sql = "call iam.user_set_password($1, $2);";
        match self.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.client.execute(
                    sql,
                    &[
                        &user_id,
                        &password
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
