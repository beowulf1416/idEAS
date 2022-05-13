/// middleware for user
use log::{ info, error, debug };

use http::header::AUTHORIZATION;

use std::task::{ Context, Poll };
use std::future::{ ready, Ready };
use std::rc::Rc;

// use auth::auth::Auth;

use actix_web::{
    HttpMessage,
    web,
    error::Error,
    dev::{
        Service, 
        Transform, 
        ServiceRequest, 
        ServiceResponse
    }
};

use futures::future::LocalBoxFuture;
use futures::executor::block_on;

use users::jwt::{
    JWT,
    Claims
};
use data::data::Data;
use users::users::Users;

// use common::user::User;
use common::email::Email;
// use auth::auth::{ Auth, Claims };


pub struct User {
}

pub struct UserMiddleware<S> {
    service: Rc<S>
}


// impl Default for User {

//     fn default() -> Self {
//         return User {};
//     }
// }


impl User {

    pub fn new() -> Self {
        return User {
        }
    }
}


impl <S, B> Transform<S, ServiceRequest> for User
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    S: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = UserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        info!("User::new_transform()");
        ready(Ok(UserMiddleware {
            // auth: self.auth.clone(),
            service: Rc::new(service)
        }))
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
    // type Future = S::Future;

    fn poll_ready(&self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(context)
    }

    // dev::forward_ready!(service);

    /*
    fn call(&self, request: ServiceRequest) -> Self::Future {
        info!("UserMiddleware::call()");

        // let service = self.service.clone();

        if request.headers().contains_key(AUTHORIZATION) {
            let header_value = request.headers().get(AUTHORIZATION).unwrap().to_str().unwrap().clone();
            let token = header_value.replace("Bearer", "").trim().to_owned();

            debug!("UserMiddleware::call(): {:?}", token);

            let jwt = request.app_data::<web::Data<JWT>>().unwrap().clone();
            if jwt.validate(&token) {
                debug!("UserMiddleware::call(): valid");

                let data = request.app_data::<web::Data<Data>>().unwrap().clone();

                if let Ok(claims) = jwt.get_claims(&token) {
                    let email = claims.get_email();
                    // let tenant_ids = claims.get_tenant_ids();
                    // let permission_ids = claims.get_permission_ids();

                    // return Box::pin(async move {
                        // info!("UserMiddleware::call() [2]");
                        // if let Ok(client) = data.get_pool().get().await {
                        //     let users = Users::new(client);
                        //     if let Ok(user) = users.get_by_email(Email::new(email).unwrap()).await {
                        //         request.extensions_mut().insert(user);
                        //     }
    
                        // }

                        if let Ok(client) = block_on(data.get_pool().get()) {
                            let users = Users::new(client);
                            if let Ok(user) = block_on(users.get_by_email(Email::new(email).unwrap())) {
                                debug!("adding user to extension: {:?}", user);
                                request.extensions_mut().insert(user);
                            }
                        }

                        // let fut = self.service.call(request);
                        // let mut res = fut.await?;
                        
                        // return Ok(res);
                    // });
                } else {
                    error!("unable to retrieve claims");
                }

                // request.extensions_mut().insert(val: T)
                // data = request.app_data::<web::Data<Data>>().unwrap().clone();

                
            } else {
                debug!("UserMiddleware::call(): token is not valid");
            }
        }        

        // let fut = self.service.call(request);
        // return Box::pin(async move {
        //     let mut res = fut.await?;

        //     info!("UserMiddleware::call() [2]");

        //     return Ok(res);
        // });
        return self.service.call(request);
    }
    */

    fn call(&self, request: ServiceRequest) -> Self::Future {
        info!("UserMiddleware::call()");

        let service = self.service.clone();
        
        return Box::pin(async move {
            if request.headers().contains_key(AUTHORIZATION) {
                let header_value = request.headers().get(AUTHORIZATION).unwrap().to_str().unwrap().clone();
                let token = header_value.replace("Bearer", "").trim().to_owned();
                let jwt = request.app_data::<web::Data<JWT>>().unwrap().clone();

                if jwt.validate(&token) {
                    if let Ok(claims) = jwt.get_claims(&token) {
                        let email = claims.get_email();

                        let data = request.app_data::<web::Data<Data>>().unwrap().clone();
                        if let Ok(client) = data.get_pool().get().await {
                            let users = Users::new(client);
                            if let Ok(user) = users.get_by_email(Email::new(email).unwrap()).await {
                                debug!("adding common::user::User to request extensions");
                                request.extensions_mut().insert(user);
                            }
                        }
                    }
                }
            }

            let res = service.call(request).await.unwrap();
            return Ok(res);
        });

    }
}