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
    issued: chrono::DateTime<Utc>
}

impl Claims {

    pub fn email(&self) -> String {
        return self.email.clone();
    }

    pub fn issued(&self) -> chrono::DateTime<Utc> {
        return self.issued.clone()
    }
}


pub enum TokenError {
    SigningError,
    ClaimError
}


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
        email: &str
    ) -> Result<String, TokenError> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();

        // standard claims
        claims.insert("iat", Utc::now().to_rfc3339());

        // custom claims
        claims.insert("email", email.to_string());

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
                let issued: DateTime<Utc> = claims["iat"].clone().parse().unwrap();
                return Ok(Claims {
                    email: claims["email"].clone(),
                    issued: issued
                });
            }
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
