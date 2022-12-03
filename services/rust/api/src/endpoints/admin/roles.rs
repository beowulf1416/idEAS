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

#[derive(Debug, Serialize, Deserialize)]
struct RoleFetchRequest {
    client_id: uuid::Uuid,
    filter: String,
    active: bool,
    items: i32,
    page: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct RoleGetRequest {
    role_id: uuid::Uuid
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
        .service(
            web::resource("get")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(role_get_get))
                .route(web::post().to(role_get_post))
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

async fn role_fetch_post(
    db: web::Data<Db>,
    params: web::Json<RoleFetchRequest>
) -> impl Responder {
    info!("role_fetch_post()");
    let error_message = "An error occured while processing this request";
    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let role_dbo = RoleDbo::new(client);

            match role_dbo.fetch(
                &params.client_id,
                &params.filter,
                &params.items,
                &params.page,
            ).await {
                Err(e) => {
                    error!("unable to retrieve roles");
                }
                Ok(roles) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully retrieved roles"),
                            Some(json!({
                                "roles": roles
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

async fn role_get_get() -> impl Responder {
    info!("role_get_get()");
    return HttpResponse::Ok().body("use POST method instead");
}

async fn role_get_post(
    db: web::Data<Db>,
    params: web::Json<RoleGetRequest>
) -> impl Responder {
    info!("role_get_post()");
    let error_message = "An error occured while processing this request";
    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let role_dbo = RoleDbo::new(client);

            match role_dbo.get(
                &params.role_id
            ).await {
                Err(e) => {
                    error!("unable to retrieve role");
                }
                Ok(role) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully retrieved role"),
                            Some(json!({
                                "role": role
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