/// JSON web Token
use log::{ info, error, debug };

use serde::{ Serialize, Deserialize };

use hmac::{Hmac, Mac};
use jwt::{ SignWithKey, VerifyWithKey, error::Error };
use sha2::Sha256;
use std::collections::BTreeMap;

use std::env;
use std::clone::Clone;
use std::str::FromStr;
use std::fs;
use actix_web::{ web };

use chrono::prelude::*;
// use serde_json::json;
use uuid::Uuid;


// #[derive(Serialize, Deserialize)]
// pub struct Permissions {
//     ids: Vec<i32>
// }


pub fn configure(cfg: &mut web::ServiceConfig) {
    // JWT configuration
    if let Ok(secret_file) = env::var("JWT_SECRET_FILE") {
        info!("reading JWT secret: {}", secret_file);
        if let Ok(secret) = fs::read_to_string(secret_file) {
            info!("jwt key: [{}]", secret);
            let jwt = JWT::new(String::from(secret.trim()));
            cfg.app_data(web::Data::new(jwt.clone()));
        } else {
            error!("unable to read JWT secret");
        }
    } else {
        error!("environment variable JWT_SECRET_FILE is empty");
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    email: String,
    tenant_id: Uuid
}

impl Claims {

    pub fn get_email(&self) -> String {
        return self.email.clone();
    }

    pub fn get_tenant_id(&self) -> Uuid {
        return self.tenant_id.clone();
    }
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JWT {
    secret: String
}


impl JWT {

    pub fn new(secret: String) -> Self {
        return JWT {
            secret: secret
        };
    }

    /// generate JWT token
    pub fn generate(
        &self,
        email: &String,
        tenant_id: &Uuid
    ) -> Result<String, String> {
        debug!("JWT::generate()");

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();

        // standard claims
        claims.insert("iat", Utc::now().to_rfc3339());

        // custom claims
        claims.insert("email", email.to_string());
        claims.insert("tenant", tenant_id.to_string());

        match claims.sign_with_key(&key) {
            Ok(token) => {
                return Ok(token);
            }
            Err(e) => {
                error!("unable to sign claims: {}", e);
                return Err(String::from("unable to generate jwt token"));
            }
        }
    }

    /// validate JWT token
    pub fn validate(&self, token: &String) -> bool {
        debug!("JWT::validate()");

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let result: Result<BTreeMap<String, String>, Error> = token.verify_with_key(&key);
        if let Err(e) = result {
            error!("ERROR JWT::validate(): {:?}", e);
            return false;
        } else {
            return true;
        }
    }

    /// retrieve claims encoded in a token
    pub fn get_claims(&self, token: &String) -> Result<Claims, String> {
        debug!("JWT::get_claims()");

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let result: Result<BTreeMap<String, String>, Error> = token.verify_with_key(&key);

        match result {
            Ok(claims) => {
                return Ok(Claims {
                    email: claims["email"].clone(),
                    tenant_id: Uuid::from_str(claims["tenant"].clone().as_str()).unwrap()
                });
            }
            Err(e) => {
                error!("ERROR JWT::get_claims(): {:?}", e);
                return Err(String::from("unable to retrieve JWT tokens"));
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use log::{ error, debug };
    use crate::jwt::JWT;

    #[test]
    fn test_generate() {
        let jwt = JWT::new(String::from("secret"));
        if let Err(e) = jwt.generate(
            String::from("email@email.com")
        ) {
            assert!(false);
        }

    }

    #[test]
    fn test_validate() {
        let jwt = JWT::new(String::from("secret"));
        match jwt.generate(String::from("email@email.com")) {
            Ok(token) => {
                assert!(jwt.validate(&token));
            }
            Err(e) => {
                error!("ERROR: {:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_get_claims() {
        let jwt = JWT::new(String::from("secret"));
        match jwt.generate(String::from("email@email.com")) {
            Ok(token) => {
                if jwt.validate(&token) {
                    match jwt.get_claims(&token) {
                        Ok(claims) => {
                            debug!("CLAIMS: {:?}", claims);
                            assert!(true);
                        }
                        Err(e) => {
                            error!("ERROR: {:?}", e);
                            assert!(false);
                        }
                    }
                } else {
                    assert!(false);
                }
            }
            Err(e) => {
                error!("ERROR: {:?}", e);
                assert!(false);
            }
        }
    }
}