use log::{ info };
use actix_web::{ web, HttpResponse, Responder };

use crate::endpoints::common::default_options;
use crate::models::permissions::Permissions;
use crate::guards::auth_guard::AuthGuard;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/test")
                .route(web::method(http::Method::OPTIONS))
                    .guard(AuthGuard::new(Permissions::UserCurrent))
                    .to(default_options)
                .route(web::get()
                    .guard(AuthGuard::new(Permissions::UserCurrent))
                    .to(get_status))
        )
    ;
}


async fn get_status() -> impl Responder {
    info!("crate::endpoints::status");

    return HttpResponse::Ok()
        .finish();
}