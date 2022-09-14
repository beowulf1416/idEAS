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

use token::JWT;
use postgres::{
    Db,
    users::Users
};


pub struct User {}

pub struct UserMiddleware<S> {
    service: Rc<S>
}

impl User {
    pub fn new() -> Self {
        return Self {};
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

            let mut result = crate::models::user::User::new(
                None,
                String::from("")
            );

            if let Some(header_value) = request.headers().get(AUTHORIZATION) {
                let token = header_value.to_str().unwrap().replace("Bearer", "").trim().to_owned();
                
                let jwt = request.app_data::<web::Data<JWT>>().unwrap().clone();
                if jwt.validate(&token) {
                    debug!("token valid");

                    if let Ok(claims) = jwt.claims(&token) {
                        let email = claims.email();

                        if let Some(db) = request.app_data::<web::Data<Db>>() {
                            if let Ok(client) = db.pool().get().await {
                                let users = Users::new(client);
                                if let Ok(user) = users.get_by_email(&email).await {
                                    let user_id = user.id;

                                    result = crate::models::user::User::new(
                                        Some(user_id),
                                        email
                                    );
                                }
                            }
                        }
                    }
                } else {
                    debug!("token invalid");
                }
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