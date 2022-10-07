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
    auth::Auth
};


#[derive(Debug, Serialize, Deserialize)]
struct AuthRegisterPostRequest {
    pub id: uuid::Uuid,
    // pub token: String,
    pub email: String
}


#[derive(Debug, Serialize, Deserialize)]
struct AuthLoginPostRequest {
    pub email: String,
    pub password: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("register")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(register_get))
                .route(web::post().to(register_post))
        )
        .service(
            web::resource("register/info")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(register_info_get))
                .route(web::post().to(register_info_post))
        )
        .service(
            web::resource("login")
                .route(web::get().to(login_get))
                .route(web::post().to(login_post))
        )
        .service(
            web::resource("logout")
                .route(web::get().to(logout_get))
                .route(web::post().to(logout_post))
        )
    ;
}



async fn register_get() -> impl Responder {
    info!("register_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn register_post(
    db: web::Data<Db>,
    params: web::Json<AuthRegisterPostRequest>
) -> impl Responder {
    info!("register_post()");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client: {:?}", e);
        }
        Ok(client) => {
            let auth = Auth::new(client);
            let id = &params.id;
            // let token = &params.token;
            let email = &params.email;

            match auth.register(
                &id,
                // &token,
                &email
            ).await {
                Err(e) => {
                    error!("unable to register email address");
                }
                Ok(_) => {
                    info!("email registered");
                    return HttpResponse::Created()
                        .json(ApiResponse::new(
                            false,
                            String::from("Successfully registered email address"),
                            None
                        ));
                }
            } 
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


async fn register_info_get() -> impl Responder {
    info!("register_info_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn register_info_post(
    db: web::Data<Db>,
    params: web::Json<AuthLoginPostRequest>
) -> impl Responder {
    info!("register_info_post()");

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


async fn login_get() -> impl Responder {
    info!("login_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn login_post(
    db: web::Data<Db>,
    params: web::Json<AuthLoginPostRequest>
) -> impl Responder {
    info!("login_post()");

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


async fn logout_get() -> impl Responder {
    info!("logout_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn logout_post() -> impl Responder {
    info!("logout_post()");
    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}