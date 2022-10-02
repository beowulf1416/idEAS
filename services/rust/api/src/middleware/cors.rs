use log::{
    info,
    error,
    debug
};

use std::task::{ Context, Poll };
use std::future::{ ready, Ready };
use futures::future::LocalBoxFuture;

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
    ACCESS_CONTROL_ALLOW_CREDENTIALS,
    ACCESS_CONTROL_EXPOSE_HEADERS
};




pub struct CORS {}

pub struct CORSMiddleware<S> {
    service: S,
}


impl CORS {
    pub fn new() -> Self {
        return CORS {};
    }
}


impl <S, B> Transform<S, ServiceRequest> for CORS
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CORSMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(CORSMiddleware {
            service
        }));
    }
}


impl <S, B> Service<ServiceRequest> for CORSMiddleware<S> 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        return self.service.poll_ready(context);
    }

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let origin = HeaderValue::from_static("*");
        let fut = self.service.call(request);
        debug!("CORSMiddleware::call() 1");

        return Box::pin(async move {
            debug!("CORSMiddleware::call() 2");

            let mut res = fut.await?;
            
            if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_ORIGIN) {
                res.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, origin);
            }

            if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_METHODS) {
                res.headers_mut().insert(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("POST, OPTIONS"));
            }

            if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_HEADERS) {
                res.headers_mut().insert(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("content-type, authorization"));
            }

            // if !res.headers().contains_key(ACCESS_CONTROL_ALLOW_CREDENTIALS) {
            //     res.headers_mut().insert(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("true"));
            // }

            if !res.headers().contains_key(ACCESS_CONTROL_EXPOSE_HEADERS) {
                // TODO: need to tighten this
                res.headers_mut().insert(ACCESS_CONTROL_EXPOSE_HEADERS, HeaderValue::from_static("authorization"));
            }

            return Ok(res);
        })
    }
}