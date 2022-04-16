/// auth
use log::{ info, error, debug };
use serde::{ Serialize, Deserialize };

// use std::env;
// use std::str::FromStr;
// use std::fs;

use deadpool_postgres::{ Pool };

use uuid::Uuid;
use common::email::Email;
use common::user::User;

use crate::jwt;
use crate::db;
use crate::email::EmailAddress;



#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    email: String
}

impl Claims {

    pub fn get_email(&self) -> String {
        return self.email.clone();
    }
}


#[derive(Clone)]
pub struct Auth {
    jwt: jwt::JWT,
    pool: Option<Pool>
}


impl Auth {

    pub fn new() -> Self {
        debug!("Auth::new()");

        let secret = jwt::configure();
        if let Ok(pool) = db::configure() {
            return Auth {
                jwt: jwt::JWT::new(secret),
                pool: Some(pool)
            }
        } else {
            return Auth {
                jwt: jwt::JWT::new(secret),
                pool: None
            }
        }
    }

    /// validate jwt token
    pub fn validate(&self, token: &String) -> bool {
        debug!("Auth::authenticate()");

        return self.jwt.validate(&token);
    }

    /// retrieve claims
    pub fn claims(&self, token: &String) -> Result<Claims, String> {
        debug!("Auth::claims()");

        if let Ok(claims) = self.jwt.get_claims(&token) {
            return Ok(Claims {
                email: claims.get_email()
            });
        } else {
            return Err(String::from("unable to retrieve claims"));
        }
    }

    /// retrieve user details
    pub async fn user(&self, email: Email) -> Result<User, String> {
        debug!("Auth::user()");

        if let Some(pool) = self.pool.clone() {
            if let Ok(client) = pool.get().await {

                let result_stmt = client.prepare_cached(
                    "select * from iam.user_get_by_email($1);"
                ).await;

                let t_email = EmailAddress::new(email);
                match result_stmt {
                    Ok(stmt) => {
                        match client.query_one(
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
            } else {
                error!("unable to retrieve database client");

                // return false;
                return Err(String::from("unable to retrieve database client"));
            }
        } else {
            error!("there is no database pool");

            // return false;
            return Err(String::from("there is no database pool"));
        }
    }
}


#[cfg(test)]
mod tests {

    use crate::jwt::JWT;
    use crate::auth::Auth;

    #[test]
    fn test_new() {
        let auth = Auth::new();
    }

    #[test]
    fn test_validate() {
        let jwt = JWT::new(String::from("secret"));
        if let Err(e) = jwt.generate(String::from("email@email.com")) {
            assert!(false);
        }

        if let Ok(token) = jwt.generate(String::from("email@email.com")) {
            let auth = Auth::new();
            assert!(auth.validate(&token));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_claims() {
        let jwt = JWT::new(String::from("secret"));
        if let Err(e) = jwt.generate(String::from("email@email.com")) {
            assert!(false);
        }

        let email_1 = String::from("email@email.com");
        if let Ok(token) = jwt.generate(email_1.clone()) {
            let auth = Auth::new();
            if auth.validate(&token) {
                if let Ok(claims) = auth.claims(&token) {
                    let email = claims.get_email();
                    assert!(email.eq(&email_1));
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
}
