/// tenants endpoints
use log::{ info, error, debug };

use actix_web::{ web, HttpRequest, HttpResponse, Responder };
use serde::{ Serialize, Deserialize };

use uuid::Uuid;

use crate::endpoints::common::default_options;
use crate::models::api_response::ApiResponse;

use tenants::tenants::Tenants;


#[derive(Debug, Serialize, Deserialize)]
struct TenantAddRequest {
    pub name: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::post().to(tenant_add_post))
        )
        .service(
            web::resource("/active")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::post().to(tenant_active_post))
        )
    ;
}

/// add tenant
async fn tenant_add_post(
    request: HttpRequest,
    tenant: web::Json<TenantAddRequest>
) -> Responder {
    info!("endpoints::tenants::tenant_add_post()");

    if let Ok(tenants) = Tenants::from_request(request).await {
        let id = Uuid::new_v4();
        let name = tenant.name;

        if let Err(e) = tenants.add(id, name).await {
            error!("unable to add tenant: {:?}", e);

            return HttpResponse::OK()
                .json(ApiResponse {
                    status: String::from("error"),
                    message: format!("unable to add tenant: {}", e),
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
    } else {
        return HttpResponse::OK()
            .json(ApiResponse {
                status: String::from("error"),
                message: String::from("error"),
                data: None
            });
    }
}

/// toggle tenant active status
async fn tenant_active_post(
    request: HttpRequest
) -> Responder {
    info!("endpoints::tenants::tenant_active_post()");

    if let Ok(tenants) = Tenants::from_request(request).await {
        return HttpResponse::Ok()
            .json(ApiResponse {
                status: String::from("success"),
                message: String::from("success"),
                data: None
            });
    } else {
        return HttpResponse::OK()
            .json(ApiResponse {
                status: String::from("error"),
                message: String::from("error"),
                data: None
            });
    }
}