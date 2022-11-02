use log::{
    info,
    debug,
    error
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

use http::{
    method::Method,
    // header::AUTHORIZATION
    header
};

use token::Token;
use common::user::User as UserObject;
use pg::{
    Db,
    DbError,
    user::User as UserDbo,
    iam::user_client::UserClient
};
use crate::extractors::user_parameter::UserParameter;


pub struct User {
    // token: Token
}

pub struct UserMiddleware<S> {
    // token: Token,
    service: Rc<S>
}

impl User {
    pub fn new(token: Token) -> Self {
        return Self {
            // token: token
        };
    }

    // pub fn token(&self) -> Token {
    //     return self.token;
    // }
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
            // token: self.token(),
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
        debug!("UserMiddleware::call() 1");

        let service = self.service.clone();

        return Box::pin(async move {
            debug!("UserMiddleware::call() 2");

            let mut result = UserObject::new(
                None,
                false,
                String::from("")
            );
            
            if request.method() == Method::POST {
                if let Some(header_value) = request.headers().get(header::AUTHORIZATION) {
                    let token_value = header_value.to_str().unwrap().replace("Bearer", "").trim().to_owned();

                    if let Some(token) = request.app_data::<web::Data<Token>>() {
                        if token.validate(&token_value) {
                            match token.claims(&token_value) {
                                Err(e) => {
                                    error!("unable to retrieve claims: {:?}", e);
                                }
                                Ok(claims) => {
                                    let email = claims.email();
                                    let client_id = claims.client_id();
                                    let issued  = claims.issued();

                                    if let Some(db) = request.app_data::<web::Data<Db>>() {
                                        if let Ok(client) = db.get_client().await {
                                            let users = UserDbo::new(client);
                                            match users.get_by_email(&email).await {
                                                Err(e) => {
                                                    error!("unable to retrieve user: {:?}", e);
                                                }
                                                Ok(u) => {
                                                    result = u.clone();
                                                    result.set_client(client_id);

                                                    if let Some(user_id) = u.id() {
                                                        match users.fetch_clients(
                                                            &user_id
                                                        ).await {
                                                            Err(e) => {
                                                                error!("unable to fetch clients: {:?}", e);
                                                            }
                                                            Ok(clients) => {
                                                                // debug!("clients {:?}", clients);
                                                                result.set_clients(clients);

                                                                // // retrieve default client
                                                                // match users.get_default_client().await {
                                                                //     Err(e) => {
                                                                //         error!("unable to retrieve default client");
                                                                //     }
                                                                //     Ok(client_id) => {
                                                                //         // debug!("client_id: {:?}", client_id);
                                                                //         result.set_client(client_id);

                                                                        match users.fetch_permissions(
                                                                            &user_id,
                                                                            &client_id
                                                                        ).await {
                                                                            Err(e) => {
                                                                                error!("unable to retrieve permissions");
                                                                            }
                                                                            Ok(permissions) => {
                                                                                debug!("permissions: {:?}", permissions);
                                                                                result.set_permissions(permissions);
                                                                            }
                                                                        }
                                                                //     }
                                                                // }
                                                            }
                                                        }
                                                    }
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
                    } else {
                        error!("unable to retrieve token object");
                    }
                } else {
                    debug!("no authorization header found: {:?}", request.headers().keys());
                }
            } else {
                debug!("request method found: {:?}", request.method());
            }


            // debug!("user: {:?}", result);
            request.extensions_mut().insert(UserParameter::new(result));

            let fut = service.call(request);
            let res = fut.await?;

            return Ok(res);
        })
    }
}