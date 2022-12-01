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
    iam::permission::Permissions as PermissionDbo
};

#[derive(Debug, Serialize, Deserialize)]
struct PermissionFilterRequest {
    filter: String,
    active: bool,
    items: i32,
    page: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct PermissionAssignedRequest {
    role_id: uuid::Uuid
}



pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /roles
    cfg
        .service(
            web::resource("fetch")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(permissions_filter_get))
                .route(web::post().to(permissions_filter_post))
        )
        .service(
            web::resource("assigned")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(permissions_assigned_get))
                .route(web::post().to(permissions_assigned_post))
        )
        .service(
            web::resource("not-assigned")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(permissions_not_assigned_get))
                .route(web::post().to(permissions_not_assigned_post))
        )
    ;
}


async fn permissions_filter_get() -> impl Responder {
    info!("permissions_filter_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn permissions_filter_post(
    db: web::Data<Db>,
    params: web::Json<PermissionFilterRequest>
) -> impl Responder {
    info!("permissions_filter_post()");
    let error_message = "an error occured while trying to process request";

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client: {:?}", e);
        }
        Ok(client) => {
            let permissions_dbo = PermissionDbo::new(client);
            match permissions_dbo.filter(
                &params.filter,
                &params.active,
                &params.items,
                &params.page
            ).await {
                Err(e) => {
                    error!("unable to retrieve permissions");
                }
                Ok(permissions) => {
                    debug!("permissions_assigned_post(): {:?}", permissions);
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully retrieved permissions"),
                            Some(json!({
                                "permissions": permissions
                            }))
                        ));
                }
            }
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from(error_message),
            None
        ));
}


async fn permissions_assigned_get() -> impl Responder {
    info!("permissions_assigned_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn permissions_assigned_post(
    db: web::Data<Db>,
    params: web::Json<PermissionAssignedRequest>
) -> impl Responder {
    info!("permissions_assigned_post()");
    let error_message = "an error occured while trying to process request";

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client: {:?}", e);
        }
        Ok(client) => {
            let permissions_dbo = PermissionDbo::new(client);
            let role_id = params.role_id;
            match permissions_dbo.get_role_permissions(&role_id).await {
                Err(e) => {
                    error!("unable to retrieve permissions");
                }
                Ok(permissions) => {
                    debug!("permissions_assigned_post(): {:?}", permissions);
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully retrieved permissions"),
                            Some(json!({
                                "permissions": permissions
                            }))
                        ));
                }
            }
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from(error_message),
            None
        ));
}


async fn permissions_not_assigned_get() -> impl Responder {
    info!("permissions_not_assigned_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn permissions_not_assigned_post(
    db: web::Data<Db>,
    params: web::Json<PermissionAssignedRequest>
) -> impl Responder {
    info!("permissions_not_assigned_post()");
    let error_message = "an error occured while trying to process request";

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client: {:?}", e);
        }
        Ok(client) => {
            let permissions_dbo = PermissionDbo::new(client);
            let role_id = params.role_id;
            match permissions_dbo.get_role_permissions_not_assigned(&role_id).await {
                Err(e) => {
                    error!("unable to retrieve permissions");
                }
                Ok(permissions) => {
                    debug!("permissions_assigned_post(): {:?}", permissions);
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully retrieved permissions"),
                            Some(json!({
                                "permissions": permissions
                            }))
                        ));
                }
            }
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from(error_message),
            None
        ));
}

