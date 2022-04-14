use log::{ info, error, debug };

use deadpool_postgres::{ Manager };
use deadpool::managed::Object;

use uuid::Uuid;

use common::email::Email;
use common::user::User;
use crate::email::EmailAddress;


/// Users
pub struct Users {
    client: Object<Manager>
}

impl Users {

    pub fn new(
        client: Object<Manager>
    ) -> Self {
        return Users {
            client: client
        };
    }

    /// add a user account
    pub async fn add(
        &self,
        id: Uuid,
        email: Email,
        password: String
    ) -> Result<(), String> {
        info!("Users::add()");

        let result_stmt = self.client.prepare_cached(
            "call iam.user_add($1, $2, $3);"
        ).await;
        let t_email = EmailAddress::new(email);
        match result_stmt {
            Ok(stmt) => {
                if let Err(e) = self.client.query(
                    &stmt,
                    &[
                        &id,
                        &t_email,
                        &password
                    ]
                ).await {
                    error!("unable to add user: {}", e);
                    return Err(String::from("unable to add user"));
                } else {
                    return Ok(());
                }
            }
            Err(e) => {
                error!("unable to prepare statement: {}", e);
                return Err(String::from("unable to prepare statement"));
            }
        }
    }

    // pub async fn get_id(
    //     &self,
    //     email: Email
    // ) -> Result<User, String> {
    //     info!("Users::get_id()");

    //     let result_stmt = self.client.prepare_cached(
    //         "call iam.user_get_id($1);"
    //     ).await;
    // }

    /// authenticate email and password combination
    pub async fn authenticate(
        &self,
        email: Email,
        password: String
    ) -> Result<bool, String> {
        info!("Users::authenticate()");

        let result_stmt = self.client.prepare_cached(
            "select * from iam.user_authenticate($1, $2);"
        ).await;

        let t_email = EmailAddress::new(email);
        match result_stmt {
            Ok(stmt) => {
                match self.client.query_one(
                    &stmt,
                    &[
                        &t_email,
                        &password
                    ]
                ).await {
                    Ok(r) => {
                        let authentic: bool = r.get("user_authenticate");
                        return Ok(authentic);
                    }
                    Err(e) => {
                        error!("ERROR: {:?}", e);
                        return Err(String::from("unable to authenticate user"));
                    }
                }
            }
            Err(e) => {
                error!("unable to prepare statement: {}", e);
                return Err(String::from("unable to prepare statement"));
            }
        }
    }

    /// toggle user active status
    pub async fn set_active(
        &self,
        id: Uuid,
        active: bool
    ) -> Result<(), String> {
        info!("Users::set_active()");

        let result_stmt = self.client.prepare_cached(
            "call iam.user_set_active($1, $2);"
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
                    error!("ERROR: {:?}", e);
                    return Err(String::from("unable to toggle user active status"));
                } else {
                    return Ok(());
                }
            }
            Err(e) => {
                error!("ERROR: {:?}", e);
                return Err(String::from("unable to prepare statement"));
            }
        }
    }

    /// retrieve user by email address
    pub async fn get_by_email(
        &self,
        email: Email
    ) -> Result<User, String> {
        info!("Users::get_by_email()");

        let result_stmt = self.client.prepare_cached(
            "select * from iam.user_get_by_email($1);"
        ).await;

        let t_email = EmailAddress::new(email);
        match result_stmt {
            Ok(stmt) => {
                match self.client.query_one(
                    &stmt,
                    &[
                        &t_email
                    ]
                ).await {
                    Ok(r) => { 
                        debug!("row: {:?}", r);

                        let id: Uuid = r.get("id");
                        let active: bool = r.get("active");
                        let email: Email = Email::new(r.get("email"));

                        return Ok(User::new(
                            id,
                            active,
                            email
                        ));
                    }
                    Err(e) => {
                        error!("Error: {:?}", e);
                        return Err(String::from("unable to retrieve user by email"));
                    }
                }
            }
            Err(e) => {
                error!("ERROR: {:?}", e);
                return Err(String::from("unable to prepare statement"));
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

    use common::email::Email;
    // use crate::email::EmailAddress;

    use std::sync::Once;
    static INIT: Once = Once::new();


    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }

    // #[test]
    #[actix_rt::test]
    async fn test_user_add() {
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
                    let email = Email::new(String::from(
                        format!("email{suffix}@email.com", suffix = suffix)
                    ));
                    let pw = String::from("thisIs1Password");

                    let users = crate::users::Users::new(pool.get().await.unwrap());
                    if let Err(e) = users.add(id, email, pw).await {
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
    async fn test_user_authenticate() {
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
                    let email = Email::new(String::from(
                        format!("email{suffix}@email.com", suffix = suffix)
                    ));
                    let pw = String::from("thisIs1Password");

                    let users = crate::users::Users::new(
                        pool.get().await.unwrap()
                    );
                    if let Ok(_) = users.add(
                        id, 
                        email.clone(), 
                        pw.clone()
                    ).await {
                        if let Err(e) = users.authenticate(
                            email.clone(), 
                            pw.clone()
                        ).await {
                            error!("error: {:?}", e);

                            assert!(false);
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
    async fn test_user_set_active() {
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
                    let email = Email::new(String::from(
                        format!("email{suffix}@email.com", suffix = suffix)
                    ));
                    let pw = String::from("thisIs1Password");

                    let users = crate::users::Users::new(
                        pool.get().await.unwrap()
                    );

                    if let Ok(_) = users.add(
                        id, 
                        email.clone(), 
                        pw.clone()
                    ).await {
                        if let Err(e) = users.set_active(
                            id.clone(), 
                            true
                        ).await {
                            error!("error: {:?}", e);

                            assert!(false);
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
    async fn test_get_by_email() {
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
                    let email = Email::new(String::from(
                        format!("email{suffix}@email.com", suffix = suffix)
                    ));
                    let pw = String::from("thisIs1Password");

                    let users = crate::users::Users::new(
                        pool.get().await.unwrap()
                    );

                    if let Ok(_) = users.add(
                        id, 
                        email.clone(), 
                        pw.clone()
                    ).await {
                        if let Err(e) = users.get_by_email(
                            email
                        ).await {
                            error!("error: {:?}", e);

                            assert!(false);
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
}