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

// use common::{
//     user::User
// };
use crate::extractors::user_parameter::UserParameter;


#[derive(Debug, Serialize, Deserialize)]
struct CurrentUserRequest {
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("current")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_current_get))
                .route(web::post().to(user_current_post))
        )
    ;
}


async fn user_current_get() -> impl Responder {
    info!("user_current_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn user_current_post(
    db: web::Data<Db>,
    user: UserParameter
) -> impl Responder {
    info!("user_current_post()");

    debug!("user: {:?}", user);

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}
