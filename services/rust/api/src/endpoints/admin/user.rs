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
    iam::user::UserDbo
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAddRequest {
    pub email: String,
    pub password: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    return cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_add_get))
                .route(web::post().to(user_add_post))
        )
    ;
}


pub async fn user_add_get() -> impl Responder {
    info!("user_add_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn user_add_post(
    db: web::Data<Db>,
    params: web::Json<UserAddRequest>
) -> impl Responder {
    info!("user_add_post()");
    let err_msg = String::from("an error occured while processing this request");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let user_dbo = UserDbo::new(client);
            match user_dbo.add(
                &params.email,
                &params.password
            ).await {
                Err(e) => {
                    error!("unable to add user record");
                }
                Ok(_) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully added user record"),
                            None
                        ));
                }
            }
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            err_msg,
            None
        ));
}