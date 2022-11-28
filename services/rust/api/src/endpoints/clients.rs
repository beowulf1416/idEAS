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

use crate::{
    guards,
    endpoints::{
        ApiResponse,
        default_options
    }
};



#[derive(Debug, Serialize, Deserialize)]
struct ClientAddRequest {
    id: uuid::Uuid,
    name: String,
    active: bool,
    description: String,
    address: String,
    country_id: i32,
    url: String
}


#[derive(Debug, Serialize, Deserialize)]
struct ClientUpdateRequest {
    id: uuid::Uuid,
    name: String,
    active: bool,
    description: String,
    address: String,
    country_id: i32,
    url: String
}


#[derive(Debug, Serialize, Deserialize)]
struct ClientGetRequest {
    id: uuid::Uuid
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

#[derive(Debug, Serialize, Deserialize)]
struct ClientMembersRequest {
    client_id: uuid::Uuid,
    active: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientMemberInviteRequest {
    client_id: uuid::Uuid,
    email: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /clients
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(client_add_get))
                .route(
                    web::post()
                    .guard(guards::permission("client.add"))
                    .to(client_add_post)
                )
        )
        .service(
            web::resource("get")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(client_get_get))
                .route(web::post().to(client_get_post))
        )
        .service(
            web::resource("update")
                .route(web::method(http::Method::OPTIONS).to(default_options))
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
        .service(
            web::resource("members")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(client_members_get))
                .route(web::post().to(client_members_post))
        )
        .service(
            web::resource("members/invite")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(client_members_invite_get))
                .route(web::post().to(client_members_invite_post))
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
                &params.active,
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


async fn client_get_get() -> impl Responder {
    info!("client_get_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn client_get_post(
    db: web::Data<Db>,
    params: web::Json<ClientGetRequest>
) -> impl Responder {
    info!("client_get_post()");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let client_dbo = ClientDbo::new(client);
            match client_dbo.get(
                &params.id
            ).await {
                Err(e) => {
                    error!("unable to fetch client");
                }
                Ok(client) => {
                    debug!("clients: {:?}", client);

                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully retrieved client information"),
                            Some(json!({
                                "client": client
                            }))
                        ));
                }
            }
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from("An error occured while trying to retrieve client information"),
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

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let client_dbo = ClientDbo::new(client);

            match client_dbo.update(
                &params.id,
                &params.name,
                &params.active,
                &params.description,
                &params.address,
                &params.country_id,
                &params.url
            ).await {
                Err(e) => {
                    error!("unable to update client record");
                }
                Ok(_) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("Client record updated"),
                            None
                        ));
                }
            }
        }
    }

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("An error occured while updating client record"),
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


async fn client_members_get() -> impl Responder {
    info!("client_members_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn client_members_post(
    db: web::Data<Db>,
    params: web::Json<ClientMembersRequest>
) -> impl Responder {
    info!("client_members_post()");
    let error_message = String::from("an error occured while trying to process this request");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let client_dbo = ClientDbo::new(client);
            match client_dbo.members(
                &params.client_id,
                &params.active
            ).await {
                Err(e) => {
                    error!("unable to retrieve client members");
                }
                Ok(members) => {
                    debug!("client members: {:?}", members);
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully retrieved client members"),
                            Some(json!({
                                "members": members
                            }))
                        ));
                }
            }
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            error_message,
            None
        ));
}


async fn client_members_invite_get() -> impl Responder {
    info!("client_members_invite_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn client_members_invite_post(
    db: web::Data<Db>,
    params: web::Json<ClientMemberInviteRequest>
) -> impl Responder {
    info!("client_members_invite_post()");
    let error_message = String::from("an error occured while trying to process this request");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let client_dbo = ClientDbo::new(client);
            match client_dbo.member_invite(
                &params.client_id,
                &params.email
            ).await {
                Err(e) => {
                    error!("unable to invite client member");
                }
                Ok(_) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully invited client member"),
                            None
                        ));
                }
            }
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            error_message,
            None
        ));
}