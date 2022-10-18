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

use pg::Db;

use crate::endpoints::{
    ApiResponse,
    default_options
};


#[derive(Debug, Serialize, Deserialize)]
struct CountriesFetchRequest {
    filter: String,
    items: i32,
    page: i32
}


#[derive(Debug, Serialize, Deserialize)]
struct CountryGet {
    id: i32
}


pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /countries
    cfg
        .service(
            web::resource("fetch")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(countries_fetch_get))
                .route(web::post().to(countries_fetch_post))
        )
        .service(
            web::resource("get")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(countries_get_get))
                .route(web::post().to(countries_get_post))
        )
    ;
}


async fn countries_fetch_get() -> impl Responder {
    info!("countries_fetch_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn countries_fetch_post(
    db: web::Data<Db>,
    params: web::Json<CountriesFetchRequest>
) -> impl Responder {
    info!("countries_fetch_post()");

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


async fn countries_get_get() -> impl Responder {
    info!("countries_get_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn countries_get_post(
    db: web::Data<Db>,
    params: web::Json<CountriesFetchRequest>
) -> impl Responder {
    info!("countries_get_post()");

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}