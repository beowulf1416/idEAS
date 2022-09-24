use log::{
    debug
};

use std::task::{ Context, Poll };
use std::future::{ ready, Ready };
use std::rc::Rc;
use futures::future::LocalBoxFuture;

use actix_web::{
    HttpMessage,
    error::Error,
    dev::{
        Service, 
        Transform, 
        ServiceRequest, 
        ServiceResponse
    },
    web
};

use http::header::{
    AUTHORIZATION
};

use token::Token;
use common::user::User as UserObject;
use pg::{
    Db,
    DbError,
    user::User as UserDbo
};


pub struct User {
    token: Token
}

pub struct UserMiddleware<S> {
    token: Token,
    service: Rc<S>
}

impl User {
    pub fn new(token: Token) -> Self {
        return Self {
            token: token
        };
    }

    pub fn token(&self) -> Token {
        return self.token;
    }
}


impl <S, B> Transform<S, ServiceRequest> for User
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    S: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = UserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(UserMiddleware {
            token: self.token(),
            service: Rc::new(service)
        }));
    }
}


impl <S, B> Service<ServiceRequest> for UserMiddleware<S> 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    S: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        return self.service.poll_ready(context);
    }

    fn call(&self, request: ServiceRequest) -> Self::Future {
        debug!("here 1");

        let service = self.service.clone();

        return Box::pin(async move {
            debug!("here 2");

            let mut result = UserObject::new(
                None,
                false,
                String::from("")
            );

            if let Some(header_value) = request.headers().get(AUTHORIZATION) {
                let token_value = header_value.to_str().unwrap().replace("Bearer", "").trim().to_owned();

                let token = self.token;
                if self.token.validate(&token_value) {
                    match token.claims(&token_value) {
                        Err(e) => {
                            error!("unable to retrieve claims: {:?}", e);
                        }
                        Ok(claims) => {
                            let email = claims.email();
                            let issued  = claims.issued();

                            if let Some(db) = request.app_data::<web::Data<Db>>() {
                                if let Ok(client) = db.get_client().await {
                                    let users = UserDbo::new(client);
                                    match users.get_by_email(&email).await {
                                        Err(e) => {
                                            error!("unable to retrieve user: {:?}", e);
                                            return Err(DbError::ClientError);
                                        }
                                        Ok(u) => {
                                            debug!("//TODO result: {:?}", u);
                                            result = u.clone();
                                        }
                                    }
                                } else {
                                    error!("unable to retrieve client object");
                                }
                            } else {
                                error!("unable to retrieve instance of Db object");
                            }
                        }
                    }
                } else {
                    error!("unable to validate token: {:?}", token_value);
                }
                
                // match token.validate(&token) {
                //     Err(e) => {
                //         error!("unable to validate token: {:?}", e);
                //     }
                //     Ok(result) => {
                //         debug!("token validation result: {:?}", result);
                //     }
                // }
                
                // let jwt = request.app_data::<web::Data<JWT>>().unwrap().clone();
                // if jwt.validate(&token) {
                //     debug!("token valid");

                //     if let Ok(claims) = jwt.claims(&token) {
                //         let email = claims.email();

                //         if let Some(db) = request.app_data::<web::Data<Db>>() {
                //             if let Ok(client) = db.get_client().await {
                //                 let user = UserDbo::new(client);

                //                 if let Ok(user) = users.get_by_email(&email).await {
                //                     let user_id = user.id;

                //                     result = crate::models::user::User::new(
                //                         Some(user_id),
                //                         email
                //                     );
                //                     debug!("result: {:?}", result);
                //                 }
                //             }
                //         }
                //     }
                // } else {
                //     debug!("token invalid");
                // }
            } else {
                debug!("no authorization header found");
            }

            request.extensions_mut().insert(result);

            let fut = service.call(request);
            let res = fut.await?;
            return Ok(res);
        })
    }
}