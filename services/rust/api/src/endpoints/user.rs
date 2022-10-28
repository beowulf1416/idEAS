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
    iam::user_client::UserClient as UserClientDbo
};

// use common::{
//     user::User
// };
use crate::extractors::user_parameter::UserParameter;


#[derive(Debug, Serialize, Deserialize)]
struct CurrentUserRequest {
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientResponse {
    pub id: uuid::Uuid,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct UserClientAddRequest {
    client_id: uuid::Uuid
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("current")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_current_get))
                .route(web::post().to(user_current_post))
        )
        .service(
            web::resource("client/add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_client_add_get))
                .route(web::post().to(user_client_add_post))
        )
    ;
}


async fn user_current_get() -> impl Responder {
    info!("user_current_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn user_current_post(
    db: web::Data<Db>,
    user: UserParameter
) -> impl Responder {
    info!("user_current_post()");

    let u = user.user();

    let mut clients: Vec<ClientResponse> = Vec::new();
    if let Some(user_clients) = u.get_clients() {
        clients = user_clients.iter().map(|c| ClientResponse {
            id: c.id,
            name: c.name.clone()
        })
        .collect();
    }

    let client_id = u.get_client();

    let mut permissions: Vec<String> = Vec::new();
    if let Some(user_permissions) = u.get_permissions() {
        permissions = user_permissions.iter().map(|up| up.name.clone())
            .collect();
    }

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            true,
            String::from("current user retrieved"),
            Some(json!({ 
                "user": {
                    "email": u.email(),
                    "name": "//TODO name",
                    "clients": clients,
                    "client": client_id,
                    "permissions": permissions
                }  
            }))
        ));
}


async fn user_client_add_get() -> impl Responder {
    info!("user_client_add_get()");
    return HttpResponse::Ok().body("use POST method instead");
}

async fn user_client_add_post(
    db: web::Data<Db>,
    user: UserParameter,
    params: web::Json<UserClientAddRequest>
) -> impl Responder {
    info!("user_client_add_post()");

    let mut error_message = "An error occured while trying to add user to client";

    let u = user.user();
    if let Some(user_id) = u.id() {
        match db.get_client().await {
            Err(e) => {
                error!("unable to retrieve client");
            }
            Ok(client) => {
                let user_client_dbo = UserClientDbo::new(client);
                match user_client_dbo.add(
                    &user_id,
                    &params.client_id
                ).await {
                    Err(e) => {
                        error_message = "unable to add user to client";
                        error!("unable to add user to client");
                    }
                    Ok(_) => {
                        return HttpResponse::Ok()
                            .json(ApiResponse::new(
                                true,
                                String::from("successfully added user to client"),
                                None
                            ));
                    }
                }
            }
        }
    } else {
        error_message = "Unable to retrieve user id";
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from(error_message),
            None
        ));
}