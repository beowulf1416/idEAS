use log::{
    info,
    error,
    debug
};


use actix_web::{ web };

use config::{
    ApplicationConfig,
    ProviderType
};


use token::Token;


#[derive(Clone)]
pub struct AuthToken {
    token: Token
}


impl AuthToken {

    pub fn new(token: Token) -> Self {
        return Self {
            token: token
        };
    }

    pub fn get_token(&self) -> &Token {
        return &self.token;
    }
}