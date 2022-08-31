use log::{
    info,
    debug,
    error
};

use serde::{
    Serialize,
    Deserialize
};

use actix_web::{
    HttpResponse, 
    Responder,
    web
};

use crate::endpoints::{
    ApiResponse
};

// use crate::services::{
//     data::Data
// };

use pg::Db;


#[derive(Debug, Serialize, Deserialize)]
struct AuthRegisterPostRequest {
    email: String
}


#[derive(Debug, Serialize, Deserialize)]
struct AuthLoginPostRequest {
    email: String,
    password: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("register")
                .route(web::get().to(register_get))
                .route(web::post().to(register_post))
        )
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



async fn register_get() -> impl Responder {
    info!("register_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn register_post(
    db: web::Data<Db>,
    params: web::Json<AuthRegisterPostRequest>
) -> impl Responder {
    info!("register_post()");

    

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


async fn login_get() -> impl Responder {
    info!("login_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn login_post(
    db: web::Data<Db>,
    params: web::Json<AuthLoginPostRequest>
) -> impl Responder {
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