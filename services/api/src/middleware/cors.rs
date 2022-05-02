/// middleware for cors headers
use log::{ info };

use std::task::{ Context, Poll };
use std::future::{ ready, Ready };

use actix_web::{
    error::Error,
    dev::{
        Service, 
        Transform, 
        ServiceRequest, 
        ServiceResponse
    }
};

use http::header::{ 
    // HeaderName,
    HeaderValue,
    // AUTHORIZATION,
    ACCESS_CONTROL_ALLOW_ORIGIN,
    ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_HEADERS, 
    // ACCESS_CONTROL_ALLOW_CREDENTIALS,
    ACCESS_CONTROL_EXPOSE_HEADERS
};

use futures::future::LocalBoxFuture;


/// CORS
pub struct CORS {}

impl Default for CORS {
    fn default() -> Self {
        return CORS {};
    }
}

impl CORS {
    pub fn new() -> Self {
        return CORS::default();
    }
}

impl <S, B> Transform<S, ServiceRequest> for CORS
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CORSMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        info!("CORS::new_transform()");
        ready(Ok(CORSMiddleware {
            service
        }))
    }
}


/// CORS Middleware Service
pub struct CORSMiddleware<S> {
    service: S,
}

impl <S, B> Service<ServiceRequest> for CORSMiddleware<S> 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(context)
    }

    // dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        info!("CORSMiddleware::call()");

        let origin = HeaderValue::from_static("*");
        let fut = self.service.call(request);

        Box::pin(async move {
            let mut res = fut.await?;
            
            if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_ORIGIN) {
                info!("modifying access_control_allow_allow_origin");
                res.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, origin);
            }

            if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_METHODS) {
                info!("modifying access_control_allow_methods");
                res.headers_mut().insert(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("POST, OPTIONS"));
            }

            if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_HEADERS) {
                info!("modifying access_control_allow_headers");
                res.headers_mut().insert(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("content-type, authorization"));
                // res.headers_mut().insert(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
            }

            // if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_CREDENTIALS) {
            //     res.headers_mut().insert(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("false"));
            // }

            if !res.headers().contains_key(ACCESS_CONTROL_EXPOSE_HEADERS) {
                // TODO: need to tighten this
                res.headers_mut().insert(ACCESS_CONTROL_EXPOSE_HEADERS, HeaderValue::from_static("authorization"));
            }

            Ok(res)
        })
    }
}