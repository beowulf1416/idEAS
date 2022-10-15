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


    // pub async fn register(
    //     &self,
    //     user_id: &uuid::Uuid,
    //     email: &str
    // ) -> Result<(), DbError> {
    //     /// register a user
    //     let t_email = crate::types::email::Email::new(email);
    //     return self.0.call_sp(
    //         "call iam.user_register($1, $2);",
    //         &[
    //             &user_id,
    //             &t_email
    //         ]
    //     ).await;
    // }

    pub async fn sign_up(
        &self,
        user_id: &uuid::Uuid,
        email: &str,
        password: &str
    ) -> Result<(), DbError> {
        let t_email = crate::types::email::Email::new(email);
        return self.0.call_sp(
            "call iam.user_signup($1, $2, $3);",
            &[
                &user_id,
                &t_email,
                &password
            ]
        ).await;
    }

    // pub async fn sign_in(
    //     &self,
    //     email: &str,
    //     password: &str
    // ) -> Result<bool, DbError>{
    //     let t_email = crate::types::email::Email::new(email);
    //     let sql = "select * from iam.user_signin($1, $2);";
    //     match self.0.get_client().prepare_cached(sql).await {
    //         Err(e) => {
    //             error!("unable to prepare statement: {} {:?}", sql, e);
    //             return Err(DbError::ClientError);
    //         }
    //         Ok(stmt) => {
    //             match self.0.get_client().query_one(
    //                 &stmt,
    //                 &[
    //                     &t_email,
    //                     &password
    //                 ]
    //             ).await {
    //                 Err(e) => {
    //                     error!("unable to execute sql: {} {:?}", sql, e);
    //                     return Err(DbError::ClientError);
    //                 }
    //                 Ok(r) => {
    //                     debug!("sign in result: {:?}", r);
    //                     return Ok(true);
    //                 }
    //             }
    //         }
    //     }
    // }

    // pub async fn register_get(
    //     &self,
    //     id: &uuid::Uuid
    // ) -> Result<common::auth::registration::Registration, DbError> {
    //     /// retrieve user registration info
    //     let sql = "select * from iam.user_register_get($1);";
    //     match self.0.get_client().prepare_cached(sql).await {
    //         Err(e) => {
    //             error!("unable to prepare statement: {} {:?}", sql, e);
    //             return Err(DbError::ClientError);
    //         }
    //         Ok(stmt) => {
    //             match self.0.get_client().query_one(
    //                 &stmt,
    //                 &[
    //                     &id
    //                 ]
    //             ).await {
    //                 Err(e) => {
    //                     error!("unable to execute sql: {} {:?}", sql, e);
    //                     return Err(DbError::ClientError);
    //                 }
    //                 Ok(r) => {
    //                     debug!("result: {:?}", r);
    //                     return Ok(common::auth::registration::Registration {
    //                         id: r.get("id"),
    //                         email: r.get("email")
    //                     });
    //                 }
    //             }
    //         }
    //     }
    // }

    // pub async fn register_complete(
    //     &self,
    //     id: &uuid::Uuid,
    //     given_name: &str,
    //     middle_name: &str,
    //     family_name: &str,
    //     prefix: &str,
    //     suffix: &str
    // ) -> Result<(), DbError> {
    //     /// flags the registration record as completed
    //     return self.0.call_sp(
    //         "call iam.user_register_complete($1, $2, $3, $4, $5, $6);",
    //         &[
    //             &id,
    //             &given_name,
    //             &middle_name,
    //             &family_name,
    //             &prefix,
    //             &suffix
    //         ]
    //     ).await;
    // }

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

    use lazy_static::lazy_static;

    // use ctor::ctor;

    use rand::Rng;
    use crate::Db;

    use crate::user::User;

    
    lazy_static!{
        static ref t_user_id: uuid::Uuid = {
            return uuid::Uuid::new_v4();
        };
    }


    // fn run_test<T>(test: T) -> ()
    //     where T: FnOnce() -> () + panic::UnwindSafe 
    // {
        


    //     let result = panic::catch_unwind(|| {
    //         test();
    //     });
    //     // teardown();
    //     assert!(result.is_ok());
    // }

    // fn setup() {
    //     if let Some(config) = config::get_configuration() {
    //         let db = Db::new(&config);
    //         match db.get_client().await {
    //             Err(e) => {
    //                 error!("unable to retrieve client {:?}", e);
    //                 assert!(false);
    //             }
    //             Ok(c) => {
    //                 client = c; 
    //             }
    //         }
    //     } else {
    //         error!("unable to retrieve database client");
    //     }
    // }

    // #[actix_rt::test] 
    // async fn test_register() {
    //     if let Some(config) = config::get_configuration() {
    //         let db = Db::new(&config);
    //         match db.get_client().await {
    //             Err(e) => {
    //                 error!("unable to retrieve client {:?}", e);
    //                 assert!(false);
    //             }
    //             Ok(client) => {
    //                 let auth = Auth::new(client);

    //                 let new_id = uuid::Uuid::new_v4();

    //                 let mut rng = rand::thread_rng();
    //                 let suffix: u8 = rng.gen();

    //                 let email = format!("email_{}@test.com", suffix);
    //                 let pw = format!("pw_{}", suffix);

    //                 match auth.register(
    //                     &new_id,
    //                     &email,
    //                     &pw
    //                 ).await {
    //                     Err(e) => {
    //                         error!("unable to register new user {:?}", e);
    //                         assert!(false);
    //                     }
    //                     Ok(_) => {
    //                         assert!(true);
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         assert!(false);
    //     }
    // }

    // #[actix_rt::test]
    // async fn test() {
    //     debug!("user_id: {}", t_user_id.to_string());
    // }

    #[actix_rt::test] 
    async fn test_sign_up() {
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

                    let email = format!("email_{}@test.com", suffix);
                    let password = "thisIsaTest88**";

                    match auth.sign_up(
                        &new_id,
                        &email,
                        &password
                    ).await {
                        Err(e) => {
                            error!("unable to sign user {:?}", e);
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

    // #[actix_rt::test] 
    // async fn test_register_get() {
    //     if let Some(config) = config::get_configuration() {
    //         let db = Db::new(&config);
    //         match db.get_client().await {
    //             Err(e) => {
    //                 error!("unable to retrieve client {:?}", e);
    //                 assert!(false);
    //             }
    //             Ok(client) => {
    //                 let auth = Auth::new(client);

    //                 let new_id = uuid::Uuid::new_v4();

    //                 let mut rng = rand::thread_rng();
    //                 let suffix: u8 = rng.gen();

    //                 let email = format!("email_{}@test.com", suffix);

    //                 match auth.register(
    //                     &new_id,
    //                     &email
    //                 ).await {
    //                     Err(e) => {
    //                         error!("unable to register new user {:?}", e);
    //                         assert!(false);
    //                     }
    //                     Ok(_) => {
    //                         match auth.register_get(
    //                             &new_id
    //                         ).await {
    //                             Err(e) => {
    //                                 error!("unable to retrieve registration info: {:?}", e);
    //                                 assert!(false);
    //                             }
    //                             Ok(r) => {
    //                                 debug!("register info: {:?}", r);
    //                                 assert!(true);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         assert!(false);
    //     }
    // }


    // #[actix_rt::test] 
    // async fn test_register_complete() {
    //     if let Some(config) = config::get_configuration() {
    //         let db = Db::new(&config);
    //         match db.get_client().await {
    //             Err(e) => {
    //                 error!("unable to retrieve client {:?}", e);
    //                 assert!(false);
    //             }
    //             Ok(client) => {
    //                 let auth = Auth::new(client);

    //                 let new_id = uuid::Uuid::new_v4();

    //                 let mut rng = rand::thread_rng();
    //                 let suffix: u8 = rng.gen();

    //                 let email = format!("email_{}@test.com", suffix);

    //                 match auth.register(
    //                     &new_id,
    //                     &email
    //                 ).await {
    //                     Err(e) => {
    //                         error!("unable to register new user {:?}", e);
    //                         assert!(false);
    //                     }
    //                     Ok(_) => {
    //                         match auth.register_complete(
    //                             &new_id,
    //                             &"test_given_name",
    //                             &"test_middle_name",
    //                             &"test_family_name",
    //                             &"test_prefix",
    //                             &"test_suffix"
    //                         ).await {
    //                             Err(e) => {
    //                                 error!("unable to complete registration: {:?}", e);
    //                                 assert!(false);
    //                             }
    //                             Ok(_) => {
    //                                 assert!(true);
    //                             }
    //                         }                            
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         assert!(false);
    //     }
    // }


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

                    // match auth.register(
                    //     &user_id,
                    //     &email
                    // ).await {
                    //     Err(e) => {
                    //         error!("unable to register new user {:?}", e);
                    //         assert!(false);
                    //     }
                    //     Ok(_) => {
                    //         match db.get_client().await {
                    //             Err(e) => {
                    //                 error!("unable to retrieve client 2 {:?}", e);
                    //                 assert!(false);
                    //             }
                    //             Ok(client_2) => {
                    //                 let password = format!("password_{}", suffix);
                    //                 let user = User::new(client_2);
                    //                 match user.set_password(
                    //                     &user_id,
                    //                     &password
                    //                 ).await {
                    //                     Err(e) => {
                    //                         error!("unable to set password {:?}", e);
                    //                         assert!(false);
                    //                     }
                    //                     Ok(_) => {
                    //                         let active = true;
                    //                         match user.set_active(
                    //                             &user_id,
                    //                             &active
                    //                         ).await {
                    //                             Err(e) => {
                    //                                 error!("unable to set user active status {:?}", e);
                    //                                 assert!(false);
                    //                             }
                    //                             Ok(_) => {
                    //                                 match auth.authenticate(
                    //                                     &email,
                    //                                     &password
                    //                                 ).await {
                    //                                     Err(e) => {
                    //                                         error!("unable to authenticate {:?}", e);
                    //                                         assert!(false);
                    //                                     }
                    //                                     Ok(authentic) => {
                    //                                         debug!("authenticate result: {:?}", authentic);
                    //                                         assert!(true);
                    //                                     }
                    //                                 }
                    //                             }
                    //                         }
                    //                     }
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                }
            }
        } else {
            assert!(false);
        }
    }
}