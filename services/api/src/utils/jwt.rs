/// JSON web Token
use log::{ info, error };

use serde::{ Serialize, Deserialize };

use hmac::{Hmac, Mac};
use jwt::{ SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

use std::env;
use std::clone::Clone;
use std::fs;
use actix_web::{ web };

use chrono::prelude::*;


pub fn configure(cfg: &mut web::ServiceConfig) {
    /// JWT configuration
    if let Ok(secret_file) = env::var("JWT_SECRET_FILE") {
        info!("reading JWT secret: {}", secret_file);
        if let Ok(secret) = fs::read_to_string(secret_file) {
            let jwt = JWT::new(secret);
            cfg.app_data(web::Data::new(jwt.clone()));
        } else {
            error!("unable to read JWT secret");
        }
    } else {
        error!("environment variable JWT_SECRET_FILE is empty");
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct JWT {
    secret: String
}


impl JWT {

    pub fn new(secret: String) -> Self {
        return JWT {
            secret: secret
        };
    }

    pub fn generate(
        &self,
        email: String
    ) -> Result<String, String> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();

        claims.insert("email", email);
        claims.insert("iat", Utc::now().to_rfc3339());
        // claims.insert("exp", );

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
}


#[cfg(test)]
mod tests {

    use crate::utils::jwt::JWT;

    #[test]
    fn test_generate() {
        let jwt = JWT::new(String::from("secret"));
        if let Err(e) = jwt.generate(String::from("email@email.com")) {
            assert!(false);
        }

    }
}