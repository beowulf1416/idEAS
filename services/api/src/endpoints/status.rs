use log::{ info, error, debug };
use actix_web::{ web, HttpRequest, HttpResponse, Responder };

use crate::endpoints::common::default_options;

use crate::guards::user_guard::AuthGuard;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/test")
                .route(web::method(http::Method::OPTIONS))
                    .guard(AuthGuard::new())
                    .to(default_options)
                .route(web::get()
                    .guard(AuthGuard::new())
                    .to(get_status))
        )
    ;
}


async fn get_status() -> impl Responder {
    info!("crate::endpoints::status");

    return HttpResponse::Ok()
        .finish();
}