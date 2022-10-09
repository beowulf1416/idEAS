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
    ApiResponse
};



#[derive(Debug, Serialize, Deserialize)]
struct ClientAddRequest {
    id: uuid::Uuid,
    name: String,
    description: String,
    address: String,
    country_id: i32,
    url: String
}


#[derive(Debug, Serialize, Deserialize)]
struct ClientUpdateRequest {
    id: uuid::Uuid,
    name: String,
    description: String,
    address: String,
    country_id: i32,
    url: String
}


#[derive(Debug, Serialize, Deserialize)]
struct ClientSetActiveRequest {
    id: uuid::Uuid,
    active: bool
}


pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /clients
    cfg
        .service(
            web::resource("add")
                .route(web::get().to(client_add_get))
                .route(web::post().to(client_add_post))
        )
        .service(
            web::resource("update")
                .route(web::get().to(client_update_get))
                .route(web::post().to(client_update_post))
        )
        .service(
            web::resource("set/active")
                .route(web::get().to(client_set_active_get))
                .route(web::post().to(client_set_active_post))
        )
    ;
}



async fn client_add_get() -> impl Responder {
    info!("client_add_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn client_add_post(
    db: web::Data<Db>,
    params: web::Json<ClientAddRequest>
) -> impl Responder {
    info!("client_add_post()");

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


async fn client_update_get() -> impl Responder {
    info!("client_update_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn client_update_post(
    db: web::Data<Db>,
    params: web::Json<ClientUpdateRequest>
) -> impl Responder {
    info!("client_update_post()");

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


async fn client_set_active_get() -> impl Responder {
    info!("client_set_active_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn client_set_active_post(
    db: web::Data<Db>,
    params: web::Json<ClientSetActiveRequest>
) -> impl Responder {
    info!("client_set_active_post()");

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}