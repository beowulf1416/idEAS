use log::{
    info,
    debug,
    error
};

use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

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
    users::UsersDbo
};

use crate::extractors::user_parameter::UserParameter;

#[derive(Debug, Serialize, Deserialize)]
struct UsersFetchRequest {
    pub filter: String,
    pub items: i32,
    pub page: i32
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("fetch")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(users_fetch_get))
                .route(web::post().to(users_fetch_post))
        )
    ;
}


async fn users_fetch_get() -> impl Responder {
    info!("users_fetch_get()");
    return HttpResponse::Ok().body("use POST method instead");
}

async fn users_fetch_post(
    db: web::Data<Db>,
    user: UserParameter,
    params: web::Json<UsersFetchRequest>
) -> impl Responder {
    info!("users_fetch_post()");
    let err_msg = String::from("an error occured while trying to process the request");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let users_dbo = UsersDbo::new(client);
            let params_page = params.page - 1;
            match users_dbo.fetch(
                &params.filter,
                &params.items,
                &params_page
            ).await {
                Err(e) => {
                    error!("unable to retrieve records");
                }
                Ok(users) => {
                    debug!("users_fetch_post: {:?}", users);
                    
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully retrieved users"),
                            Some(json!({
                                "users": users
                            }))
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