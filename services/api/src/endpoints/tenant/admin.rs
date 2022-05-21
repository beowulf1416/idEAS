// tenant admin endpoints
use log::{ info, error, debug };

use actix_web::{ web, HttpRequest, HttpResponse, Responder };
use serde::{ Serialize, Deserialize };

use uuid::Uuid;

// use data::data::Data;
use tenants::tenants::Tenants;

use crate::endpoints::common::default_options;
use crate::models::api_response::{ ApiResponse, ApiResponseStatus};

use crate::models::permissions::Permissions;
use crate::guards::auth_guard::AuthGuard;


#[derive(Debug, Serialize, Deserialize)]
struct TenantAddRequest {
    pub name: String
}

#[derive(Serialize, Deserialize)]
struct TenantActiveRequest {
    pub tenant_id: Uuid,
    pub active: bool
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(
                    web::post()
                        .guard(AuthGuard::new(Permissions::TenantAdd))
                        .to(tenant_add_post))
        )
        .service(
            web::resource("/active")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(
                    web::post()
                        .guard(AuthGuard::new(Permissions::TenantSetActive))
                        .to(tenant_set_active_post)
                )  
        )
    ;
}

/// add tenant endpoint
async fn tenant_add_post(
    request: HttpRequest,
    params: web::Json<TenantAddRequest>
) -> impl Responder {
    info!("endpoints::tenant::admin::tenant_add_post()");

    let mut error_msg = String::from("unable to add tenant");

    match Tenants::from_request(&request).await {
        Ok(tenants_db) => {
            let tenant_id = Uuid::new_v4();
            let tenant_name = params.name.clone();

            if let Err(e) = tenants_db.add(
                &tenant_id,
                &tenant_name
            ).await {
                error!("unable to add tenant: {:?}", e);
                error_msg = format!("unable to add tenant: {}", e);
            } else {
                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: ApiResponseStatus::Success,
                        message: String::from("added tenant"),
                        data: None
                    });
            }
        }
        Err(e) => {
            error!("unable to retrieve tenants object: {:?}", e);
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse {
            status: ApiResponseStatus::Error,
            message: error_msg,
            data: None
        });
}


/// set tenant active status
async fn tenant_set_active_post(
    request: HttpRequest,
    params: web::Json<TenantActiveRequest>
) -> impl Responder {
    info!("endpoints::tenant::admin::tenant_set_active_post()");

    let mut error_msg = String::from("unable to set tenant active status");

    match Tenants::from_request(&request).await {
        Ok(tenants_db) => {
            let tenant_id = params.tenant_id.clone();
            let active = params.active.clone();

            if let Err(e) = tenants_db.active(
                &tenant_id,
                &active
            ).await {
                error!("unable to set tenant active status: {:?}", e);
                error_msg = format!("unable to add tenant: {}", e);
            } else {
                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: ApiResponseStatus::Success,
                        message: String::from("successfully set tenant active status"),
                        data: None
                    });
            }
        }
        Err(e) => {
            error!("unable to retrieve tenants object: {:?}", e);
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse {
            status: ApiResponseStatus::Error,
            message: error_msg,
            data: None
        });
}
