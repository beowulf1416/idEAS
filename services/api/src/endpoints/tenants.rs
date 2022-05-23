/// tenants endpoints
use log::{ info, error, debug };

use actix_web::{ web, HttpRequest, HttpResponse, Responder };
use serde::{ Serialize, Deserialize };

use uuid::Uuid;

use crate::endpoints::common::default_options;
use crate::models::api_response::{ ApiResponse, ApiResponseStatus };

use tenants::tenants::Tenants;
use common::email::Email;


#[derive(Debug, Serialize, Deserialize)]
struct TenantAddUserRequest {
    pub tenant_id: Uuid,
    pub user_id: Uuid
}


#[derive(Serialize, Deserialize)]
struct TenantUserActiveRequest {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub active: bool
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/user/add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::post().to(tenant_add_user))
        )
        .service(
            web::resource("/user/active")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::post().to(tenant_user_set_active))
        )
    ;
}


// add a user to tenant
async fn tenant_add_user(
    request: HttpRequest,
    params: web::Json<TenantAddUserRequest>
) -> impl Responder {
    info!("endpoints::tenants::tenant_add_user");

    let mut error_msg = String::from("unable to add user to tenant");

    let user_id = params.user_id.clone();
    let tenant_id = params.tenant_id.clone();

    match Tenants::from_request(&request).await {
        Ok(tenants_db) => {
            if let Err(e) = tenants_db.add_user(
                &tenant_id,
                &user_id
            ).await {
                error!("unable to add user to tenant: {:?}", e);
                error_msg = format!("unable to add user to tenant: {}", e);
            } else {
                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: ApiResponseStatus::Success,
                        message: format!("successfully added user to tenant"),
                        data: None
                    });
            }
        }
        Err(e) => {
            error!("unable to retrieve tenants db object: {:?}", e);
            error_msg = format!("unable to retrieve tenants db object: {}", e);
        }
    }


    return HttpResponse::InternalServerError()
        .json(ApiResponse {
            status: ApiResponseStatus::Error,
            message: error_msg,
            data: None
        });
}



// set user-tenant active status
async fn tenant_user_set_active(
    request: HttpRequest,
    params: web::Json<TenantUserActiveRequest>
) -> impl Responder {
    info!("endpoints::tenants::tenant_user_set_active");

    let mut error_msg = String::from("unable to set user-tenant active status");

    let user_id = params.user_id.clone();
    let tenant_id = params.tenant_id.clone();
    let active = params.active.clone();

    match Tenants::from_request(&request).await {
        Ok(tenants_db) => {
            if let Err(e) = tenants_db.set_active(
                &tenant_id,
                &user_id,
                &active
            ).await {
                error!("unable to set user-tenant active status: {:?}", e);
                error_msg = format!("unable to set user-tenant active status: {}", e);
            } else {
                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: ApiResponseStatus::Success,
                        message: format!("successfully set user-tenant active status"),
                        data: None
                    });
            }
        }
        Err(e) => {
            error!("unable to set user-tenant active status: {:?}", e);
            error_msg = format!("unable to set user-tenant active status: {}", e);
        }
    }


    return HttpResponse::InternalServerError()
        .json(ApiResponse {
            status: ApiResponseStatus::Error,
            message: error_msg,
            data: None
        });
}