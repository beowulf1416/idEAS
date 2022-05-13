/**
 * user related endpoints
 */

use log::{ info, error, debug };

use serde::{ Serialize, Deserialize };
use serde_json::{
    // from_str,
    json
};

use actix_web::{
    web, 
    HttpMessage,
    HttpRequest,
    HttpResponse,
    Responder
};

use http::header::AUTHORIZATION;

use uuid::Uuid;

use crate::models::api_response::ApiResponse;
use crate::endpoints::common::default_options;
use crate::guards::auth_guard::AuthGuard;

use common::email::Email;
use data::data::Data;
use users::{
    jwt::JWT,
    users::Users,
    user_param::UserParam
};



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

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordChangeRequest {
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
                .route(
                    web::method(http::Method::OPTIONS)
                    // .guard(AuthGuard::new(String::from("permission.test")))
                    .to(default_options)
                )
                .route(
                    web::post()
                    // TODO correct permission
                    .guard(AuthGuard::new(String::from("permission.test")))
                    .to(get_user_post)
                )
        )
        .service(
            web::resource("/password")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::post().to(password_change))
        )
    ;
}

/// signup endpoint
async fn signup_post(
    _request: HttpRequest,
    data: web::Data<Data>,
    sign_up: web::Json<SignUpRequest>
) -> impl Responder {
    info!("endpoints::user::signup_post()");
    info!("sign_up: {:?}", sign_up);

    match data.get_pool().get().await {
        Ok(client) => {
            info!("client obtained");

            let users = Users::new(client);
            let id = Uuid::new_v4();

            if let Err(e) = users.add(
                id,
                Email::new(sign_up.email.clone()).unwrap(),
                sign_up.password.clone()
            ).await {
                error!("unable to add user: {}", e);

                return HttpResponse::InternalServerError()
                    .json(ApiResponse {
                        status: String::from("error"),
                        message: format!("unable to add user: {}", e),
                        data: None
                    });
            } else {
                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: String::from("success"),
                        message: String::from("success"),
                        data: None
                    });
            }
        }
        Err(e) => {
            error!("error obtaining client: {:?}", e);

            return HttpResponse::Ok()
                .json(ApiResponse {
                    status: String::from("error"),
                    message: format!("{}", e),
                    data: None
                });
        }
    }
}


/// signin endpoint
async fn signin_post(
    request: HttpRequest,
    data: web::Data<Data>,
    jwt: web::Data<JWT>,
    sign_in: web::Json<SignInRequest>
) -> impl Responder {
    info!("endpoints::user::signin_post()");
    // info!("sign_in: {:?}", sign_in);

    match data.get_pool().get().await {
        Ok(client) => {
            let email = Email::new(sign_in.email.clone()).unwrap();

            let users = Users::new(client);
            if let Ok(authentic) = users.authenticate(
                email.clone(),
                sign_in.password.clone()
            ).await {
                if authentic {
                    // if let Some(user) = request.extensions().get::<common::user::User>() {
                    //     let user_id = user.get_id();
                        match jwt.generate(
                            sign_in.email.clone()
                        ) {
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
                                        message: format!("{}", e),
                                        data: None
                                    });
                            }
                        }
                    // } else {
                    //     error!("no user object in request extension");

                    //     return HttpResponse::Ok()
                    //     .json(ApiResponse {
                    //         status: String::from("error"),
                    //         message: String::from("error"),
                    //         data: None
                    //     });
                    // }
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
                    message: format!("{}", e),
                    data: None
                });
        }
    }
}

/// get user endpoint
async fn get_user_post(
    _request: HttpRequest,
    data: web::Data<Data>,
    user: UserParam
) -> impl Responder {
    info!("endpoints::user::get_user_post()");

    debug!("USER: {:?}", user);
    let u = user.to_user();

    let user_id = u.get_id();
    let user_email = u.get_email();

    match data.get_pool().get().await {
        Ok(client) => {
            let users = Users::new(client);

            if let Ok(tenants) = users.get_tenants(u.get_id()).await {
                debug!("tenants: {:?}", tenants);

                if tenants.len() > 0 {
                    // get default client
                    let default_tenant = (&tenants[0..1])[0].clone();
                    debug!("default tenant: {:?}", default_tenant);

                    // retrieve user permissions
                    let default_tenant_id = default_tenant.0;
                    if let Ok(permissions) = users.get_user_permissions(
                        user_id,
                        &default_tenant_id
                    ).await {
                        debug!("permissions: {:?}", permissions);

                        return HttpResponse::Ok()
                            .json(ApiResponse {
                                status: String::from("success"),
                                message: String::from("success"),
                                data: Some(json!({
                                    "email": user_email,
                                    "tenants": tenants,
                                    "tenant_default": default_tenant,
                                    "permissions": permissions
                                }))
                            });

                    } else {
                        error!("unable to retrieve user permissions");
                    }
                } else {
                    error!("user is not associated with any tenant");
                }
            } else {
                error!("unable to retrieve tenants");
            }
        }
        Err(e) => {
            error!("ERROR: {}", e);
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse {
            status: String::from("error"),
            message: String::from("error"),
            data: None
        });
}


/// change password
async fn password_change(
    _request: HttpRequest,
    data: web::Data<Data>,
    user: UserParam,
    params: web::Json<PasswordChangeRequest>
) -> impl Responder {
    info!("endpoints::user::password_change()");

    let u = user.to_user();
    let user_id = u.get_id();

    match data.get_pool().get().await {
        Ok(client) => {
            let users = Users::new(client);
            if let Err(e) = users.set_password(
                user_id, 
                params.password.clone()
            ).await {
                error!("unable to set password: {}", e);
            } else {
                info!("password updated");
                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: String::from("success"),
                        message: String::from("success"),
                        data: None
                    });
            }
        }
        Err(e) => {
            error!("unable to get database client: {}", e);
        }
    }
    
    return HttpResponse::InternalServerError()
        .json(ApiResponse {
            status: String::from("error"),
            message: String::from("error"),
            data: None
        });
}