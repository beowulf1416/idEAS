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

use pg::{
    Db,
    DbError,
    client::client::Client as ClientDbo
};

use crate::endpoints::{
    ApiResponse,
    default_options
};



#[derive(Debug, Serialize, Deserialize)]
struct ClientAddRequest {
    id: uuid::Uuid,
    name: String,
    description: String,
    address: String,
    country_id: i32,
    url: String
}


#[derive(Debug, Serialize, Deserialize)]
struct ClientUpdateRequest {
    id: uuid::Uuid,
    name: String,
    description: String,
    address: String,
    country_id: i32,
    url: String
}


#[derive(Debug, Serialize, Deserialize)]
struct ClientSetActiveRequest {
    id: uuid::Uuid,
    active: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientsFetchRequest{
    filter: String,
    active: bool,
    items: i32,
    page: i32
}


pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /clients
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(client_add_get))
                .route(web::post().to(client_add_post))
        )
        .service(
            web::resource("update")
                .route(web::get().to(client_update_get))
                .route(web::post().to(client_update_post))
        )
        .service(
            web::resource("set/active")
                .route(web::get().to(client_set_active_get))
                .route(web::post().to(client_set_active_post))
        )
        .service(
            web::resource("fetch")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(client_fetch_get))
                .route(web::post().to(client_fetch_post))
        )
    ;
}



async fn client_add_get() -> impl Responder {
    info!("client_add_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn client_add_post(
    db: web::Data<Db>,
    params: web::Json<ClientAddRequest>
) -> impl Responder {
    info!("client_add_post()");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let client_dbo = ClientDbo::new(client);

            match client_dbo.add(
                &params.id,
                &params.name,
                &params.description,
                &params.address,
                &params.country_id,
                &params.url
            ).await {
                Err(e) => {
                    error!("unable to add client");
                }
                Ok(_) => {
                    return HttpResponse::Created()
                        .json(ApiResponse::new(
                            true,
                            String::from("Client added"),
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


async fn client_update_get() -> impl Responder {
    info!("client_update_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn client_update_post(
    db: web::Data<Db>,
    params: web::Json<ClientUpdateRequest>
) -> impl Responder {
    info!("client_update_post()");

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


async fn client_set_active_get() -> impl Responder {
    info!("client_set_active_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn client_set_active_post(
    db: web::Data<Db>,
    params: web::Json<ClientSetActiveRequest>
) -> impl Responder {
    info!("client_set_active_post()");

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


async fn client_fetch_get() -> impl Responder {
    info!("client_fetch_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn client_fetch_post(
    db: web::Data<Db>,
    params: web::Json<ClientsFetchRequest>
) -> impl Responder {
    info!("client_fetch_post()");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let client_dbo = ClientDbo::new(client);
            match client_dbo.fetch(
                &params.filter,
                &params.active,
                &params.items,
                &params.page
            ).await {
                Err(e) => {
                    error!("unable to fetch clients");
                }
                Ok(clients) => {
                    debug!("clients: {:?}", clients);

                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("Clients"),
                            Some(json!({
                                "clients": clients
                            }))
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