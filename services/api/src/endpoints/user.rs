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

use crate::models::api_response::{ ApiResponse, ApiResponseStatus };
use crate::models::permissions::Permissions;
use crate::endpoints::common::default_options;
use crate::guards::auth_guard::AuthGuard;

use common::email::Email;
use data::data::Data;
use users::{
    jwt::JWT,
    users::Users,
    user_param::UserParam
};
use tenants::tenants::Tenants;



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

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionsRequest {
    pub tenant_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct TenantSelectRequest {
    pub tenant_id: Uuid
}


#[derive(Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String
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
                    .to(default_options)
                )
                .route(
                    web::post()
                    .guard(AuthGuard::new(Permissions::UserCurrent))
                    .to(get_user_post)
                )
        )
        .service(
            web::resource("/password")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(
                    web::post()
                    .guard(AuthGuard::new(Permissions::UserCurrent))
                    .to(password_change)
                )
        )
        .service(
            web::resource("/tenants")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(
                    web::post()
                        .guard(AuthGuard::new(Permissions::UserCurrent))
                        .to(get_tenants_post)
                )
        )
        .service(
            web::resource("/permissions")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(
                    web::post()
                        .guard(AuthGuard::new(Permissions::UserCurrent))
                        .to(get_permissions_post)
                )
        )
        .service(
            web::resource("/tenant/select")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(
                    web::post()
                        .guard(AuthGuard::new(Permissions::UserCurrent))
                        .to(tenant_select_post)
                )
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
                        status: ApiResponseStatus::Error,
                        message: format!("unable to add user: {}", e),
                        data: None
                    });
            } else {
                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: ApiResponseStatus::Success,
                        message: String::from("success"),
                        data: None
                    });
            }
        }
        Err(e) => {
            error!("error obtaining client: {:?}", e);

            return HttpResponse::Ok()
                .json(ApiResponse {
                    status: ApiResponseStatus::Error,
                    message: format!("{}", e),
                    data: None
                });
        }
    }
}


/// signin endpoint
async fn signin_post(
    _request: HttpRequest,
    data: web::Data<Data>,
    jwt: web::Data<JWT>,
    sign_in: web::Json<SignInRequest>
) -> impl Responder {
    info!("endpoints::user::signin_post()");

    let mut error_msg = String::from("unable to sign in user");

    match data.get_pool().get().await {
        Ok(client) => {
            let email = Email::new(sign_in.email.clone()).unwrap();

            let users = Users::new(client);
            if let Ok(authentic) = users.authenticate(
                email.clone(),
                sign_in.password.clone()
            ).await {
                if authentic {
                    if let Ok(client) = data.get_pool().get().await {
                        let tenants = Tenants::new(client);
                        if let Ok(default_tenant_id) = tenants.default_tenant_id().await {
                            match jwt.generate(
                                &sign_in.email,
                                &default_tenant_id
                            ) {
                                Ok(token) => {
                                    return HttpResponse::Ok()
                                        .append_header((AUTHORIZATION, format!("Bearer {}", token)))
                                        .json(ApiResponse {
                                            status: ApiResponseStatus::Success,
                                            message: String::from("success"),
                                            data: None
                                        });
                                }
                                Err(e) => {
                                    error!("unable to generate token: {:?}", e);
                                    error_msg = format!("unable to generate token: {}", e);
                                }
                            }
                        } else {
                            error!("unable to retrieve default tenant_id");
                            error_msg = format!("unable to retrieve default tenant_id");
                        }
                    }
                } else {
                    error!("user is not valid");
                    error_msg = format!("user failed authentication");
                }
            }
        }
        Err(e) => {
            error!("error obtaining client: {:?}", e);
            error_msg = format!("unable to sign in user: {}", e);
        }
    }

    return HttpResponse::Ok()
        .json(ApiResponse {
            status: ApiResponseStatus::Error,
            message: error_msg,
            data: None
        });
}

/// get user endpoint
async fn get_user_post(
    request: HttpRequest,
    data: web::Data<Data>,
    user: UserParam
) -> impl Responder {
    info!("endpoints::user::get_user_post()");

    // debug!("USER: {:?}", user);
    let u = user.to_user();

    let user_id = u.get_id();
    let user_email = u.get_email();
    
    let extensions = request.extensions();
    let current_tenant_id = extensions.get::<Uuid>().unwrap();
    let permissions = extensions.get::<Vec<String>>().unwrap();

    match data.get_pool().get().await {
        Ok(client) => {
            let users = Users::new(client);

            if let Ok(tenants) = users.get_tenants(&user_id).await {
                debug!("tenants: {:?}", tenants);

                if tenants.len() > 0 {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            status: ApiResponseStatus::Success,
                            message: String::from("success"),
                            data: Some(json!({
                                "email": user_email,
                                "tenants": tenants,
                                "tenant_current": current_tenant_id,
                                "permissions": permissions
                            }))
                        });
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
            status: ApiResponseStatus::Error,
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
                        status: ApiResponseStatus::Success,
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
            status: ApiResponseStatus::Error,
            message: String::from("error"),
            data: None
        });
}


/// retrieve tenants
async fn get_tenants_post(
    _request: HttpRequest,
    data: web::Data<Data>,
    user_param: UserParam
) -> impl Responder {
    info!("endpoints::users::get_tenants_post()");

    let error_msg = String::from("unable to retrieve user tenants");

    let user = user_param.to_user();
    let user_id = user.get_id();

    match data.get_pool().get().await {
        Ok(client) => {
            let users = Users::new(client);
            if let Ok(tenants) = users.get_tenants(&user_id).await {

                let data_tenants: Vec<Tenant> = tenants.iter().map(|t| Tenant {
                    id: t.0.clone(),
                    name: t.1.clone()
                }).collect();

                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: ApiResponseStatus::Success,
                        message: String::from("successfully retrieved tenants"),
                        data: Some(json!({
                            "tenants": data_tenants
                        }))
                    });
            }
        }
        Err(e) => {
            error!("unable to retrieve tenants: {:?}", e);
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse {
            status: ApiResponseStatus::Error,
            message: error_msg,
            data: None
        });
}

/// retrieve permissions
async fn get_permissions_post(
    _request: HttpRequest,
    data: web::Data<Data>,
    user_param: UserParam,
    params: web::Json<PermissionsRequest>
) -> impl Responder {
    info!("endpoints::users::get_permissions_post()");

    let error_msg = String::from("unable to retrieve user permissions");

    let user = user_param.to_user();
    let user_id = user.get_id();

    let tenant_id = params.tenant_id.clone();

    match data.get_pool().get().await {
        Ok(client) => {
            let users = Users::new(client);
            if let Ok(permissions) = users.get_user_permissions(
                &user_id, 
                &tenant_id
            ).await {
                let perms: Vec<String> = permissions.iter().map(|p| p.1.clone()).collect();

                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: ApiResponseStatus::Success,
                        message: String::from("successfully retrieved permissions"),
                        data: Some(json!({
                            "permissions": perms
                        }))
                    });
            }
        }
        Err(e) => {
            error!("unable to retrieve permissions: {:?}", e);
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse {
            status: ApiResponseStatus::Error,
            message: error_msg,
            data: None
        });
}

async fn tenant_select_post(
    _request: HttpRequest,
    data: web::Data<Data>,
    jwt: web::Data<JWT>,
    user_param: UserParam,
    params: web::Json<TenantSelectRequest>
) -> impl Responder {
    info!("endpoints::user::tenant_select_post()");

    let mut error_msg = String::from("unable to current tenant");

    let user = user_param.to_user();
    let email = user.get_email().clone();

    let tenant_id = params.tenant_id.clone();

    // check if tenant_id is valid
    if let Ok(client) = data.get_pool().get().await {
        let tenants_db = Tenants::new(client);
        if let Ok(_tenant) = tenants_db.get_tenant(&tenant_id).await {
            match jwt.generate(
                &email.get_email_str(),
                &tenant_id
            ) {
                Ok(token) => {
                    return HttpResponse::Ok()
                        .append_header((AUTHORIZATION, format!("Bearer {}", token)))
                        .json(ApiResponse {
                            status: ApiResponseStatus::Success,
                            message: String::from("success"),
                            data: None
                        });
                }
                Err(e) => {
                    error!("unable to generate token: {:?}", e);
                    error_msg = format!("unable to generate token: {}", e);
                }
            }
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse {
            status: ApiResponseStatus::Error,
            message: error_msg,
            data: None
        });
}