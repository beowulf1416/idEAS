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


pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /clients
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(category_add_get))
                .route(web::post().to(category_add_post))
        )
    ;
}


async fn category_add_get() -> impl Responder {
    info!("category_add_get()");
    return HttpResponse::Ok().body("use POST method instead");
}

async fn category_add_post(
    db: web::Data<Db>
) -> impl Responder {
    info!("category_add_post()");

    let error_message = String::from("an error occured while trying to process this request");

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            error_message,
            None
        ));

}