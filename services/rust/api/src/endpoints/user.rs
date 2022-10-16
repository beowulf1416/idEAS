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
    DbError
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



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("current")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_current_get))
                .route(web::post().to(user_current_post))
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

    debug!("user: {:?}", u);

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
            String::from("Service is up. version: 1.0.0.0.dev"),
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
