use log::{
    info,
    debug,
    error
};

use actix_web::{
    HttpResponse, 
    Responder,
    web
};

use crate::endpoints::{
    ApiResponse
};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("login")
                .route(web::get().to(login_get))
                .route(web::post().to(login_post))
        )
        .service(
            web::resource("login")
                .route(web::get().to(logout_get))
                .route(web::post().to(logout_post))
        )
    ;
}



async fn login_get() -> impl Responder {
    info!("login_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn login_post() -> impl Responder {
    info!("login_post()");
    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


async fn logout_get() -> impl Responder {
    info!("logout_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn logout_post() -> impl Responder {
    info!("logout_post()");
    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}