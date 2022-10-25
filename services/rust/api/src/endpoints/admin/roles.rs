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
    ApiResponse,
    default_options
};

use pg::{
    Db,
    DbError
};


#[derive(Debug, Serialize, Deserialize)]
struct RoleAddRequest {
    id: uuid::Uuid,
    name: String,
    description: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /roles
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(role_add_get))
                .route(web::post().to(role_add_post))
        )
    ;
}


pub async fn role_add_get() -> impl Responder {
    info!("role_add_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn role_add_post(
    db: web::Data<Db>,
    params: web::Json<RoleAddRequest>
) -> impl Responder {
    info!("role_add_post()");



    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}