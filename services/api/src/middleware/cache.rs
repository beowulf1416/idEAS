/// middleware for cache control headers
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
    HeaderValue,
    CACHE_CONTROL,
    EXPIRES
};

use futures::future::LocalBoxFuture;


pub struct Cache {}

impl Cache {

    pub fn new() -> Self {
        return Cache {

        };
    }
}

pub struct CacheMiddleware<S> {
    service: S,
}


impl <S, B> Service<ServiceRequest> for CacheMiddleware<S> 
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
        info!("CacheMiddleware::call()");

        let fut = self.service.call(request);
        return Box::pin(async move {
            let mut res = fut.await?;

            res.headers_mut().insert(CACHE_CONTROL, HeaderValue::from_static("no-cache, must-revalidate"));
            res.headers_mut().insert(EXPIRES, HeaderValue::from_static("16 Jul 1978 00:00:00 GMT"));

            return Ok(res);
        });
    }
}



impl <S, B> Transform<S, ServiceRequest> for Cache
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CacheMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        info!("Cache::new_transform()");
        ready(Ok(CacheMiddleware {
            service
        }))
    }
}