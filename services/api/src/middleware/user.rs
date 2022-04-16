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

use deadpool_postgres::{ Pool };

// use crate::utils::jwt::JWT;
// use common::user::User;
use common::email::Email;
use auth::auth::{ Auth, Claims };


pub struct User {
    auth: Auth
}

pub struct UserMiddleware<S> {
    auth: Auth,
    service: S
}


// impl Default for User {

//     fn default() -> Self {
//         return User {};
//     }
// }


impl User {

    pub fn new(auth: Auth) -> Self {
        // return User::default();
        return User {
            auth: auth
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
            auth: self.auth.clone(),
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

        let auth = self.auth.clone();
    
        if request.headers().contains_key(AUTHORIZATION) {
            let header_value = request.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
            token = header_value.replace("Bearer", "");

            if auth.validate(&token) {
                debug!("authenticated");

                if let Ok(claims) = auth.claims(&token) {
                    email = claims.get_email();
                }
            }
        }

        // let req = request.clone();

        // let empty_str = String::from("");
        // let mut token = String::from("");
        // if request.headers().contains_key(AUTHORIZATION) {
        //     // debug!("AUTHORIZATION: {}", request.headers().get(AUTHORIZATION).unwrap().to_str().unwrap());
        //     debug!("Authorization provided");

        //     let header_value = request.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
        //     token = header_value.replace("Bearer", "");
        // }

        // let mut email = String::from("");
        // if token.ne(&empty_str) {
        //     if let Some(jwt) = request.app_data::<web::Data<JWT>>() {
        //         if jwt.validate(&token) {
        //             if let Ok(claims) = jwt.get_claims(&token) {
        //                 email = claims.get_email();
        //             }
        //         }
        //     }
        // }

        // let mut pool: Pool;
        // if email.ne(&empty_str) {
            // if let Some(p) = request.app_data::<web::Data<Pool>>() {
            //     pool = *(p.get_ref());
            // }
        // }

        // let origin = HeaderValue::from_static("*");

        
        
        let fut = self.service.call(request);

        Box::pin(async move {
            let mut res = fut.await?;

            info!("UserMiddleware::call() [2]");

            if email.ne(&empty_str) {
                if let Ok(user) = auth.user(Email::new(email)).await {
                    debug!("auth.user() succeeded");

                    // request.app_data(web::Data::new(User));
                }
            }
            
            

            // if token.ne(&empty_str) {
            //     if let Some(jwt) = request.app_data::<web::Data<JWT>>() {
            //         debug!("DEBUG: {:?}", jwt);
            //         if jwt.validate(&token) {
            //             if let Ok(claims) = jwt.get_claims(token) {
            //                 let email = claims.get_email();
    
            //                 if let Some(pool) = request.app_data::<web::Data<Pool>>() {
            //                     // if let Ok(client) = pool.get().await {
    
            //                     // }
            //                 } else {
            //                     error!("unable to retrieve database pool");
            //                 }
    
            //                 // let user = User::new();
            //             } else {
            //                 debug!("JWT claims error");
            //             }
            //         } else {
            //             debug!("JWT token not valid");
            //         }
            //     } else {
            //         error!("unable to retrieve jwt");
            //     }
            // }

            
            // if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_ORIGIN) {
            //     info!("modifying access_control_allow_allow_origin");
            //     res.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, origin);
            // }

            // if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_METHODS) {
            //     info!("modifying access_control_allow_methods");
            //     res.headers_mut().insert(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("POST, OPTIONS"));
            // }

            // if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_HEADERS) {
            //     info!("modifying access_control_allow_headers");
            //     res.headers_mut().insert(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("Content-Type, Authorization"));
            // }

            // if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_CREDENTIALS) {
            //     res.headers_mut().insert(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("false"));
            // }

            // if !res.headers().contains_key(ACCESS_CONTROL_EXPOSE_HEADERS) {
            //     // TODO: need to tighten this
            //     res.headers_mut().insert(ACCESS_CONTROL_EXPOSE_HEADERS, HeaderValue::from_static("*"));
            // }

            Ok(res)
        })
    }
}