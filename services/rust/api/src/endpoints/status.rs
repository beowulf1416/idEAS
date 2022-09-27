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

// use crate::classes::app_config::ApplicationConfig;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(status_get))
                .route(web::post().to(status_post))
        )
    ;
}


pub async fn status_get(
    // config: web::Data<ApplicationConfig>
) -> impl Responder {
    info!("status_get()");

    // debug!("application config: {:?}", config);

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


pub async fn status_post() -> impl Responder {
    info!("status_post()");
    return HttpResponse::Ok()
        .json(ApiResponse {
            success: false,
            message: String::from("Service is up. version: 1.0.0.0.dev"),
            data: None
        });
}