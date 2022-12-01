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
    iam::role::Role as RoleDbo
};


#[derive(Debug, Serialize, Deserialize)]
struct RoleAddRequest {
    id: uuid::Uuid,
    client_id: uuid::Uuid,
    name: String,
    description: String
}

#[derive(Debug, Serialize, Deserialize)]
struct RoleUpdateRequest {
    id: uuid::Uuid,
    client_id: uuid::Uuid,
    name: String,
    description: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /roles
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(role_add_get))
                .route(web::post().to(role_add_post))
        )
        .service(
            web::resource("update")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(role_update_get))
                .route(web::post().to(role_update_post))
        )
        .service(
            web::resource("fetch")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(role_fetch_get))
                .route(web::post().to(role_fetch_post))
        )
    ;
}


async fn role_add_get() -> impl Responder {
    info!("role_add_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn role_add_post(
    db: web::Data<Db>,
    params: web::Json<RoleAddRequest>
) -> impl Responder {
    info!("role_add_post()");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let role_dbo = RoleDbo::new(client);

            match role_dbo.add(
                &params.id,
                &params.client_id,
                &params.name,
                &params.description,
            ).await {
                Err(e) => {
                    error!("unable to add role");
                }
                Ok(_) => {
                    return HttpResponse::Created()
                        .json(ApiResponse::new(
                            true,
                            String::from("Role added"),
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


async fn role_update_get() -> impl Responder {
    info!("role_update_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn role_update_post(
    db: web::Data<Db>,
    params: web::Json<RoleUpdateRequest>
) -> impl Responder {
    info!("role_update_post()");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let role_dbo = RoleDbo::new(client);

            match role_dbo.update(
                &params.id,
                &params.client_id,
                &params.name,
                &params.description,
            ).await {
                Err(e) => {
                    error!("unable to update role");
                }
                Ok(_) => {
                    return HttpResponse::Created()
                        .json(ApiResponse::new(
                            true,
                            String::from("Role update"),
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

async fn role_fetch_get() -> impl Responder {
    info!("role_fetch_get()");
    return HttpResponse::Ok().body("use POST method instead");
}

async fn role_fetch_post() -> impl Responder {
    info!("role_fetch_post()");
    return HttpResponse::Ok().body("//TODO");
}