use log::{
    info,
    error,
    debug
};

use hmac::{
    Hmac,
    Mac
};

use jwt::{
    SignWithKey,
    VerifyWithKey,
    error
};

use sha2::Sha256;
use std::collections::BTreeMap;
use chrono::prelude::*;
use std::clone::Clone;

// use std::str::FromStr;


pub struct Claims {
    email: String,
    client_id: uuid::Uuid,
    issued: chrono::DateTime<Utc>
}

impl Claims {

    pub fn email(&self) -> String {
        return self.email.clone();
    }

    pub fn client_id(&self) -> uuid::Uuid {
        return self.client_id.clone();
    }

    pub fn issued(&self) -> chrono::DateTime<Utc> {
        return self.issued.clone()
    }
}


#[derive(Debug, PartialEq)]
pub enum TokenError {
    SigningError,
    ClaimError
}


#[derive(Clone)]
pub struct Token {
    secret: String
}

impl Token {
    pub fn new(secret: &str) -> Self {
        return Self {
            secret: String::from(secret)
        };
    }

    pub fn generate(
        &self,
        email: &str,
        client_id: &uuid::Uuid
    ) -> Result<String, TokenError> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();

        // standard claims
        claims.insert("iat", Utc::now().to_rfc3339());

        // custom claims
        claims.insert("email", email.to_string());
        claims.insert("client_id", client_id.to_string());

        match claims.sign_with_key(&key) {
            Ok(token) => {
                return Ok(token);
            }
            Err(e) => {
                error!("unable to sign claims: {:?}", e);
                return Err(TokenError::SigningError);
            }
        }
    }

    pub fn validate(
        &self,
        token: &String
    ) -> bool {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let result: Result<BTreeMap<String, String>, error::Error> = token.verify_with_key(&key);
        
        if let Err(e) = result {
            error!("unable to validate token: {:?}", e);
            return false;
        } else {
            return true;
        }
    }

    pub fn claims(
        &self,
        token: &String
    ) -> Result<Claims, TokenError> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let result: Result<BTreeMap<String, String>, error::Error> = token.verify_with_key(&key);

        match result {
            Err(e) => {
                error!("unable to retrieve claims: {:?}", e);
                return Err(TokenError::ClaimError);
            }
            Ok(claims) => {
                if let Ok(client_id) = uuid::Uuid::parse_str(&claims["client_id"].clone()) {
                    let issued: DateTime<Utc> = claims["iat"].clone().parse().unwrap();
                    return Ok(Claims {
                        email: claims["email"].clone(),
                        client_id: client_id,
                        issued: issued
                    });
                } else {
                    return Err(TokenError::ClaimError);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::env;

    use std::sync::Once;
    static INIT: Once = Once::new();

    use super::*;


    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }


    #[test]
    fn test_generate() {
        initialize();

        let token = Token::new("secret");
        match token.generate("test@test.com") {
            Err(e) => {
                error!("unable to generate token: {:?}", e);
                assert!(false);
            }
            Ok(sz) => {
                if sz == "jwt_token" {
                    assert!(true);
                } else {
                    error!("token is incorrect: {:?}", sz);
                    assert!(false);
                }
            }
        }
    }

    fn test_validate() {
        initialize();

        let token = Token::new("secret");
        if !token.validate(&String::from("jwt_token")) {
            error!("unable to generate token");
            assert!(false);
        }
    }
}
