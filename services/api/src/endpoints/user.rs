/**
 * user related endpoints
 */

use log::{ info, error };

use serde::{ Serialize, Deserialize };
use actix_web::{ web, HttpRequest, HttpResponse, Responder };
use actix_http::header::{self, HeaderMap, HeaderValue};

use http::header::AUTHORIZATION;

use uuid::Uuid;
use deadpool_postgres::{ Pool };

use crate::models::api_response::ApiResponse;
// use crate::data::user::User;
use crate::endpoints::common::default_options;

use common::email::Email;
use users::users::Users;

use crate::utils::jwt::JWT;


#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/signup")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::post().to(signup_post))
        )
        .service(
            web::resource("/signin")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::post().to(signin_post))
        )
        .service(
            web::resource("/current")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::post().to(get_user_post))   
        )
    ;
}

/// signup endpoint
async fn signup_post(
    _request: HttpRequest,
    pool: web::Data<Pool>,
    sign_up: web::Json<SignUpRequest>
) -> impl Responder {
    info!("endpoints::user::signup_post()");
    info!("sign_up: {:?}", sign_up);

    match pool.get().await {
        Ok(client) => {
            info!("client obtained");

            let users = Users::new(client);
            let id = Uuid::new_v4();
            if let Ok(s) = users.add(
                id,
                Email::new(sign_up.email.clone()),
                sign_up.password.clone()
            ).await {
                info!("user signed up");
            }

            return HttpResponse::Ok()
                .json(ApiResponse {
                    status: String::from("success"),
                    message: String::from("success"),
                    data: None
                });
        }
        Err(e) => {
            error!("error obtaining client: {:?}", e);

            return HttpResponse::Ok()
                .json(ApiResponse {
                    status: String::from("error"),
                    message: String::from("error"),
                    data: None
                });
        }
    }
}


/// signin endpoint
async fn signin_post(
    _request: HttpRequest,
    pool: web::Data<Pool>,
    jwt: web::Data<JWT>,
    sign_in: web::Json<SignInRequest>
) -> impl Responder {
    info!("endpoints::user::signin_post()");
    // info!("sign_in: {:?}", sign_in);

    match pool.get().await {
        Ok(client) => {
            let users = Users::new(client);
            if let Ok(authentic) = users.authenticate(
                Email::new(sign_in.email.clone()),
                sign_in.password.clone()
            ).await {
                if authentic {
                    // generate jwt token
                    match jwt.generate(sign_in.email.clone()) {
                        Ok(token) => {
                            return HttpResponse::Ok()
                            .append_header((AUTHORIZATION, format!("Bearer {}", token)))
                            .json(ApiResponse {
                                status: String::from("success"),
                                message: String::from("success"),
                                data: None
                            });
                        }
                        Err(e) => {
                            error!("unable to generate token: {}", e);
                            
                            return HttpResponse::Ok()
                                .json(ApiResponse {
                                    status: String::from("error"),
                                    message: String::from("error"),
                                    data: None
                                });
                        }
                    }
                } else {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            status: String::from("error"),
                            message: String::from("error"),
                            data: None
                        });
                }
            } else {
                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: String::from("error"),
                        message: String::from("error"),
                        data: None
                    });
            }    
        }
        Err(e) => {
            error!("error obtaining client: {:?}", e);

            return HttpResponse::Ok()
                .json(ApiResponse {
                    status: String::from("error"),
                    message: String::from("error"),
                    data: None
                });
        }
    }
}

/// get user endpoint
async fn get_user_post(
    _request: HttpRequest,
    pool: web::Data<Pool>
) -> impl Responder {
    info!("endpoints::user::get_user_post()");

    return HttpResponse::Ok()
        .json(ApiResponse {
            status: String::from("error"),
            message: String::from("error"),
            data: None
        });
}