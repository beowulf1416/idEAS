/// user extractor
use log::{ info, error, debug };

// use std::error::Error;
// use std::convert::Into;
use std::pin::Pin;
use std::write;

use http::header::AUTHORIZATION;

use actix_web::{
    dev::Payload,
    http::StatusCode, 
    web, 
    Error, 
    // error::ErrorUnauthorized,
    // body::BoxBody,
    HttpRequest, 
    HttpResponse,
    FromRequest,
    ResponseError
};

use actix_http:: {
    Response
};

use deadpool_postgres::{ Manager, Pool };


use futures::{
    Future
};
use futures::future::{ok, err, ready, Ready};

use users::users::Users;
use common::email::Email;

use crate::utils::jwt::JWT;

// use common::user::User;

#[derive(Debug)]
pub enum AuthError {
    Error_1,
    Error_2
}


impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AuthError::Error_1 => write!(f, "error 1"),
            AuthError::Error_2 => write!(f, "error 2")
        }
    }
}

impl ResponseError for AuthError {

    fn status_code(&self) -> StatusCode {
        match *self {
            AuthError::Error_1 => StatusCode::OK,
            AuthError::Error_2 => StatusCode::OK
        }
    }
}

#[derive(Debug)]
pub struct UserParam {
    id: uuid::Uuid,
    active: bool,
    email: Email
}


// impl FromRequest for User {
//     // type Error: Into<Error>;
//     type Error = AuthError;
//     // type Future: Future<Output = Result<Self, Self::Error>>;
//     type Future = Ready<Result<Self, Self::Error>>;

//     fn from_request(request: &HttpRequest, payload: &mut Payload) -> Self::Future {
//         debug!("extractors::user::from_request()");
//         if request.headers().contains_key(AUTHORIZATION) {
//             if let Ok(auth_value) = request.headers().get(AUTHORIZATION).unwrap().to_str() {
//                 let token_value = auth_value.replace("Bearer", "");
//                 if token_value.is_empty() {
//                     return ready(Err(AuthError::Error_1));
//                 } else {

//                     if let Ok(client) = request.app_data::<web::Data<Pool>>() {

//                     }

//                     return ready(Ok(User {
//                         token: token_value
//                     }));
//                 }
//             } else {
//                 return ready(Err(AuthError::Error_1));
//             }
//         } else {
//             return ready(Err(AuthError::Error_1));
//         }
//     }
// }

impl FromRequest for UserParam {
    type Error = AuthError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(request: &HttpRequest, payload: &mut Payload) -> Self::Future {
        debug!("endpoints::users::User::from_request()");

        let mut email = Email::new(String::from(""));
        if request.headers().contains_key(AUTHORIZATION) {
            if let Ok(auth_value) = request.headers().get(AUTHORIZATION).unwrap().to_str() {
                let token_value = auth_value.replace("Bearer", "").trim().to_owned();
                if token_value.is_empty() {
                    error!("authorization token is empty");
                } else {
                    if let Some(jwt) = request.app_data::<web::Data<JWT>>() {
                        if jwt.validate(&token_value) {
                            if let Ok(claims) = jwt.get_claims(&token_value) {
                                email = Email::new(claims.get_email()).unwrap();
                            } else {
                                error!("unable to retrieve JWT claims");
                            }
                        } else {
                            error!("JWT token is invalid");
                        }
                    } else {
                        error!("unable to retrieve JWT object");
                    }
                }
            } else {
                error!("unable to get authorization token");
            }
        }

        let pool = request.app_data::<web::Data<Pool>>().unwrap().clone();

        return Box::pin(async move {
            let users = Users::new(pool.get().await.unwrap());


            // if let Ok(users) = Users::new(pool.get().await.unwrap()) {
                if let Ok(user) = users.get_by_email(email).await {
                    return Ok(UserParam {
                        id: user.get_id(),
                        active: user.get_active(),
                        email: user.get_email()
                    });
                } else {
                    error!("unable to retrieve user object");

                    return Err(AuthError::Error_1);
                }
            // } else {
            //     error!("unable to retrieve users data object");

            //     return Err(AuthError::Error_1);
            // }
        });
    }
}