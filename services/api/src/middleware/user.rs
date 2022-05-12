/// middleware for user
use log::{ info, error, debug };

use http::header::AUTHORIZATION;

use std::task::{ Context, Poll };
use std::future::{ ready, Ready };

// use auth::auth::Auth;

use actix_web::{
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

// use deadpool_postgres::{ Pool };

// use crate::utils::jwt::JWT;
use users::jwt::JWT;
// use common::user::User;
use common::email::Email;
// use auth::auth::{ Auth, Claims };


pub struct User {
}

pub struct UserMiddleware<S> {
    service: S
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
            service: service
        }))
    }
}




impl <S, B> Service<ServiceRequest> for UserMiddleware<S> 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(context)
    }

    // dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        info!("UserMiddleware::call()");

        let empty_str = String::from("");
        let mut token = String::from("");
        let mut email = String::from("");

        // let auth = self.auth.clone();
    
        if request.headers().contains_key(AUTHORIZATION) {
            let header_value = request.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
            let token = header_value.replace("Bearer", "").trim().to_owned();

            debug!("UserMiddleware::call(): {:?}", token);

            let jwt = request.app_data::<web::Data<JWT>>().unwrap().clone();
            if jwt.validate(&token) {
                debug!("UserMiddleware::call(): valid");

                // request.extensions_mut().insert(val: T)

                
            } else {
                debug!("UserMiddleware::call(): token is not valid");
            }
        }        
        
        let fut = self.service.call(request);

        Box::pin(async move {
            let mut res = fut.await?;

            info!("UserMiddleware::call() [2]");

            return Ok(res);
        })
    }
}