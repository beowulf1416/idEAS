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
    user::User as UserDbo,
    iam::user_client::UserClient as UserClientDbo
};

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

#[derive(Debug, Serialize, Deserialize)]
struct UserClientSetRequest {
    client_id: uuid::Uuid
}

#[derive(Debug, Serialize, Deserialize)]
struct UserProfileUpdateRequest {
    people_id: uuid::Uuid,
    given_name: String,
    middle_name: String,
    family_name: String,
    prefix: String,
    suffix: String
}

#[derive(Debug, Serialize, Deserialize)]
struct UserSetPasswordRequest {
    new_password: String
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
        .service(
            web::resource("client/set")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_client_set_get))
                .route(web::post().to(user_client_set_post))
        )
        .service(
            web::resource("profile/get")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_profile_get))
                .route(web::post().to(user_profile_post))
        )
        .service(
            web::resource("profile/update")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_profile_update_get))
                .route(web::post().to(user_profile_update_post))
        )
        .service(
            web::resource("profile/pw")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_password_set_get))
                .route(web::post().to(user_password_set_post))
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

    let mut error_message = String::from("An error occured while trying to add user to client");

    let u = user.user();
    debug!("user_client_add_post() {:?}", u);


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
                        error_message = String::from("unable to add user to client");
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
        error_message = String::from("Please sign in");

        return HttpResponse::Unauthorized()
            .json(ApiResponse::new(
                false,
                error_message,
                None
            ));
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from(error_message),
            None
        ));
}


async fn user_client_set_get() -> impl Responder {
    info!("user_client_set_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn user_client_set_post(
    db: web::Data<Db>,
    user: UserParameter,
    params: web::Json<UserClientSetRequest>
) -> impl Responder {
    info!("user_client_set_post()");

    let mut error_message = String::from("An error occured while trying to set current client");

    let u = user.user();
    if let Some(user_id) = u.id() {
        
    } else {
        error_message = String::from("Please sign in");

        return HttpResponse::Unauthorized()
            .json(ApiResponse::new(
                false,
                error_message,
                None
            ));
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            error_message,
            None
        ));
}



async fn user_profile_get() -> impl Responder {
    info!("user_profile_get()");
    return HttpResponse::Ok().body("use POST method instead");
}

async fn user_profile_post(
    db: web::Data<Db>,
    user: UserParameter
) -> impl Responder {
    info!("user_profile_post()");

    let mut error_message = String::from("An error occured while trying to retrieve user profile");

    let u = user.user();
    if let Some(user_id) = u.id() {
        match db.get_client().await {
            Err(e) => {
                error!("unable to retrieve client");
            }
            Ok(client) => {
                let user_dbo = UserDbo::new(client);

                if let Ok(Some(people_id)) = user_dbo.get_people_id(&user_id).await {
                    match user_dbo.get_profile(
                        &people_id
                    ).await {
                        Err(e) => {
                            error!("unable to retrieve user profile: {:?}", e);
                        }
                        Ok(r) => {
                            debug!("profile: {:?}", r);
                            return HttpResponse::Ok()
                                .json(ApiResponse::new(
                                    true,
                                    String::from("successfully retrieved user profile"),
                                    Some(json!({
                                        "people": {
                                            "people_id": Some(people_id),
                                            "given_name": "given_name",
                                            "middle_name": "middle_name",
                                            "family_name": "family_name",
                                            "prefix": "prefix",
                                            "suffix": "suffix"
                                        }
                                    }))
                                ));
                        }
                    }
                } else {
                    // no people id associated with this user
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("There is no profile associated with this user"),
                            Some(json!({
                                "people_id": "",
                                "given_name": "",
                                "middle_name": "",
                                "family_name": "",
                                "prefix": "",
                                "suffix": ""
                            }))
                        ));
                }
            }
        }
    } else {
        error_message = String::from("Please sign in");

        return HttpResponse::Unauthorized()
            .json(ApiResponse::new(
                false,
                error_message,
                None
            ));
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            error_message,
            None
        ));
}


async fn user_profile_update_get() -> impl Responder {
    info!("user_profile_update_get()");
    return HttpResponse::Ok().body("use POST method instead");
}

async fn user_profile_update_post(
    db: web::Data<Db>,
    user: UserParameter,
    params: web::Json<UserProfileUpdateRequest>
) -> impl Responder {
    info!("user_profile_update_post()");

    let mut error_message = String::from("An error occured while trying to retrieve user profile");

    let u = user.user();
    if let Some(user_id) = u.id() {
        match db.get_client().await {
            Err(e) => {
                error!("unable to retrieve client");
            }
            Ok(client) => {
                let user_dbo = UserDbo::new(client);

                let mut people_id = params.people_id;
                if let Ok(Some(t_people_id)) = user_dbo.get_people_id(&user_id).await {
                    people_id = t_people_id;
                }

                match user_dbo.update_profile(
                    &user_id,
                    &people_id,
                    &params.given_name,
                    &params.middle_name,
                    &params.family_name,
                    &params.prefix,
                    &params.suffix
                ).await {
                    Err(e) => {
                        error!("unable to update profile: {:?}", e);
                    }
                    Ok(_) => {
                        debug!("successfully updated user profile");
                        return HttpResponse::Ok()
                            .json(ApiResponse::new(
                                true,
                                String::from("successfully updated user profile"),
                                None
                            ));
                    }
                }
            }
        }
    } else {
        error_message = String::from("Please sign in");

        return HttpResponse::Unauthorized()
            .json(ApiResponse::new(
                false,
                error_message,
                None
            ));
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            error_message,
            None
        ));
}


async fn user_password_set_get() -> impl Responder {
    info!("user_password_set_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn user_password_set_post(
    db: web::Data<Db>,
    user: UserParameter,
    params: web::Json<UserSetPasswordRequest>
) -> impl Responder {
    info!("user_password_set_post()");

    let mut error_message = String::from("An error occured while trying to retrieve user profile");

    let u = user.user();
    if let Some(user_id) = u.id() {
        match db.get_client().await {
            Err(e) => {
                error!("unable to retrieve client");
            }
            Ok(client) => {
                let user_dbo = UserDbo::new(client);
                match user_dbo.set_password(
                    &user_id,
                    &params.new_password
                ).await {
                    Err(e) => {
                        error!("unable to set user password: {:?}", e);
                    }
                    Ok(_) => {
                        return HttpResponse::Ok()
                            .json(ApiResponse::new(
                                true,
                                String::from("successfully set user password"),
                                None
                            ));
                    }
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
