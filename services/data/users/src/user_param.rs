/// user
use log::{ info, error, debug };

use std::write;

use http::header::AUTHORIZATION;

use actix_web::{
    dev::Payload,
    http::StatusCode, 
    web, 
    Error, 
    HttpRequest, 
    HttpResponse,
    FromRequest,
    ResponseError
};

use futures::future::{ok, err, ready, Ready};

// use http::status::StatusCode;

use std::pin::Pin;
use futures::Future;

use deadpool_postgres::{ Manager, Pool };

use common::email::Email;
use common::user::User;

use crate::jwt::JWT;
use crate::users::Users;


#[derive(Debug)]
pub enum UserError {
    InternalServerError
}

impl std::fmt::Display for UserError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UserError::InternalServerError => write!(f, "internal server error")
        }
    }
}


impl ResponseError for UserError {

    fn status_code(&self) -> StatusCode {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
}


#[derive(Debug)]
pub struct UserParam {
    id: uuid::Uuid,
    active: bool,
    email: Email
}

impl UserParam {

    pub fn to_user(&self) -> User {
        return User::new(
            self.id, 
            self.active, 
            self.email.clone()
        );
    }
}


//https://stackoverflow.com/questions/63308246/how-to-use-async-code-in-actix-web-extractors
impl FromRequest for UserParam {
    type Error = UserError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(request: &HttpRequest, payload: &mut Payload) -> Self::Future {
        debug!("users::user::UserParam::from_request()");

        let pool = request.app_data::<web::Data<Pool>>().unwrap().clone();

        if let Some(header_value) = request.headers().get(AUTHORIZATION) {
            if let Ok(header_str) = header_value.to_str() {
                let token = String::from(header_str.replace("Bearer", "").trim());
                if !token.is_empty() {
                    if let Some(jwt) = request.app_data::<web::Data<JWT>>() {
                        if jwt.validate(&token) {
                            if let Ok(claims) = jwt.get_claims(&token) {
                                let email_str = claims.get_email();

                                if let Ok(email) = Email::new(String::from(email_str)) {
                                    return Box::pin(async move {
                                        if let Ok(client) = pool.get().await {
                                            let users = Users::new(client);
                                            // let user = users.get_by_email(Email::new(String::from("email@email.com")).unwrap()).await.unwrap();
                                            let user = users.get_by_email(email).await.unwrap();
                                            return Ok(UserParam {
                                                id: user.get_id(),
                                                active: true,
                                                email: user.get_email()
                                            });
                                        } else {
                                            return Err(UserError::InternalServerError);
                                        }
                                    });
                                } else {
                                    error!("email is not valid");
                                }
                            }
                        } else {
                            error!("token is not valid");
                        }
                    } else {
                        error!("unable to obtain JWT object");
                    }
                } else {
                    error!("token is empty");
                }
            }
        } else {
            error!("Authorization header is empty or not provided");
        }


        return Box::pin(async move {
            return Err(UserError::InternalServerError);
        });
    }
}