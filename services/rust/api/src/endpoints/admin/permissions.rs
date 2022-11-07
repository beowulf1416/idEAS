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
    DbError,
    iam::permission::Permissions as PermissionDbo
};


pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /roles
    cfg
        .service(
            web::resource("assigned")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(permissions_assigned_get))
                .route(web::post().to(permissions_assigned_post))
        )
        .service(
            web::resource("not-assigned")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(permissions_not_assigned_get))
                .route(web::post().to(permissions_not_assigned_post))
        )
    ;
}


pub async fn permissions_assigned_get() -> impl Responder {
    info!("permissions_assigned_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn permissions_assigned_post(
    db: web::Data<Db>
) -> impl Responder {
    info!("permissions_assigned_post()");
    let error_message = "an error occured while trying to process request";

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from(error_message),
            None
        ));
}


pub async fn permissions_not_assigned_get() -> impl Responder {
    info!("permissions_not_assigned_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn permissions_not_assigned_post(
    db: web::Data<Db>
) -> impl Responder {
    info!("permissions_not_assigned_post()");
    let error_message = "an error occured while trying to process request";

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from(error_message),
            None
        ));
}

