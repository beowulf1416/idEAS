/// middleware for user
use log::{ info, debug };

use http::header::AUTHORIZATION;

use std::task::{ Context, Poll };
use std::future::{ ready, Ready };
use std::rc::Rc;

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

use users::jwt::{
    JWT,
    // Claims
};
use data::data::Data;
use users::users::Users;

use common::email::Email;


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
                        let tenant_id = claims.get_tenant_id();

                        debug!("adding tenant_id to request extensions");
                        request.extensions_mut().insert(tenant_id);

                        let data = request.app_data::<web::Data<Data>>().unwrap().clone();
                        if let Ok(client) = data.get_pool().get().await {
                            let users = Users::new(client);
                            if let Ok(user) = users.get_by_email(Email::new(email).unwrap()).await {
                                let user_id = user.get_id();

                                debug!("adding common::user::User to request extensions");
                                request.extensions_mut().insert(user);
                                    
                                if let Ok(permissions) = users.get_user_permissions(
                                    &user_id,
                                    &tenant_id
                                ).await {
                                    debug!("UserMiddleware::call() permissions: {:?}", permissions);
                                    let p: Vec<String> = permissions.clone().iter().map(|p| p.1.clone()).collect();
                                    
                                    debug!("adding permissions to request extensions");
                                    request.extensions_mut().insert(p);
                                }
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