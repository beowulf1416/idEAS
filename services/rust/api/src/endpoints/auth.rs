use log::{
    info,
    debug,
    error
};

use std::sync::{ Mutex };

use serde::{
    Serialize,
    Deserialize
};

use actix_web::{
    HttpResponse, 
    Responder,
    web
};

use http::header::AUTHORIZATION;

use kafka::producer::{
    Producer, 
    Record, 
    RequiredAcks
};

use crate::endpoints::{
    ApiResponse,
    default_options
};

use config::{
    ApplicationConfig
};

use pg::{
    Db,
    DbError,
    auth::Auth,
    user::User as UserDbo,
};


#[derive(Debug, Serialize, Deserialize)]
struct AuthRegisterPostRequest {
    pub id: uuid::Uuid,
    // pub token: String,
    pub email: String,
    pub password: String
}


#[derive(Debug, Serialize, Deserialize)]
struct AuthVerifyRequest {
    pub id: uuid::Uuid
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthRegisterInfoPostRequest {
    pub id: uuid::Uuid
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthLoginPostRequest {
    pub email: String,
    pub password: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("sign-up")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(sign_up_get))
                .route(web::post().to(sign_up_post))
        )
        .service(
            web::resource("verify")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(verify_get))
                .route(web::post().to(verify_post))
        )
        // .service(
        //     web::resource("register/info")
        //         .route(web::method(http::Method::OPTIONS).to(default_options))
        //         .route(web::get().to(register_info_get))
        //         .route(web::post().to(register_info_post))
        // )
        .service(
            web::resource("sign-in")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(sign_in_get))
                .route(web::post().to(sign_in_post))
        )
        .service(
            web::resource("sign-out")
                .route(web::get().to(sign_out_get))
                .route(web::post().to(sign_out_post))
        )
    ;
}



async fn sign_up_get() -> impl Responder {
    info!("sign_up_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn sign_up_post(
    cfg: web::Data<ApplicationConfig>,
    tokenizer: web::Data<token::Token>,
    db: web::Data<Db>,
    producer: web::Data<Mutex<Producer>>,
    params: web::Json<AuthRegisterPostRequest>
) -> impl Responder {
    info!("sign_up_post()");

    let id = uuid::Uuid::new_v4();

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client: {:?}", e);
        }
        Ok(client) => {
            let auth = Auth::new(client);
            let email = &params.email;
            let pw = &params.password;

            match auth.sign_up(
                &id,
                &email,
                &pw
            ).await {
                Err(e) => {
                    error!("unable to sign up using that email address: {:?}", e);
                }
                Ok(_) => {
                    info!("user registered");

                    let body = format!(r#"
<h1>this is a test</h1>
<p>Click <a title="Click here" href="{base_url}/auth/register/verify/{token}">{base_url}/auth/register/verify/{token}</a> to continue registration</p>
"#,
token = id,
base_url = cfg.base_url
);

                    let mail = common::mail::Mail {
                        to: email.to_string(),
                        subject: "registration".to_owned(),
                        body: body.to_owned()
                    };

                    let mut p = producer.lock().unwrap();
                    if let Err(e) = p.send(&Record::from_value("mailer", serde_json::to_string(&mail).unwrap())) {
                        error!("an error occured while trying to add to queue: {:?}", e);
                    }

                    match db.get_client().await {
                        Err(e) => {
                            error!("unable to retrieve client: {:?}", e);
                        }
                        Ok(client_2) => {
                            let user_dbo = UserDbo::new(client_2);
                            if let Ok(default_client_id) = user_dbo.get_default_client().await {
                                match tokenizer.generate(&email, &default_client_id) {
                                    Err(e) => {
                                        error!("unable to generate token: {:?}", e);
                                    }
                                    Ok(token) => {
                                        return HttpResponse::Created()
                                            .append_header((AUTHORIZATION, format!("Bearer {}", token)))
                                            .json(ApiResponse::new(
                                                true,
                                                String::from("Successfully registered email address"),
                                                None
                                            ));
                                    }
                                }
                            }
                        }
                    }
                }
            } 
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from("An error occured while trying to sign up"),
            None
        ));
}


async fn verify_get() -> impl Responder {
    info!("verify_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn verify_post(
    db: web::Data<Db>,
    params: web::Json<AuthVerifyRequest>
) -> impl Responder {
    info!("verify_post()");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client: {:?}", e);
        }
        Ok(client) => {
            let auth = Auth::new(client);

            let user_id = &params.id;
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from("An error occured while trying to verify user"),
            None
        ));
}


async fn sign_in_get() -> impl Responder {
    info!("sign_in_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn sign_in_post(
    tokenizer: web::Data<token::Token>,
    db: web::Data<Db>,
    params: web::Json<AuthLoginPostRequest>
) -> impl Responder {
    info!("sign_in_post()");

    let mut error_message = String::from("an error occured while trying to respond to this request");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client: {:?}", e);
        }
        Ok(client) => {
            let auth = Auth::new(client);
            let email = &params.email;
            let pw = &params.password;

            match auth.authenticate(
                &email,
                &pw
            ).await {
                Err(e) => {
                    error!("unable to sign in: {:?}", e);
                }
                Ok(authenticated) => {
                    if authenticated {
                        match db.get_client().await {
                            Err(e) => {
                                error!("unable to retrieve client: {:?}", e);
                            }
                            Ok(client_2) => {
                                let user_dbo = UserDbo::new(client_2);
                                if let Ok(default_client_id) = user_dbo.get_default_client().await {
                                    match tokenizer.generate(&email, &default_client_id) {
                                        Err(e) => {
                                            error!("unable to generate token: {:?}", e);
                                        }
                                        Ok(token) => {
                                            return HttpResponse::Ok()
                                                .append_header((AUTHORIZATION, format!("Bearer {}", token)))
                                                .json(ApiResponse::new(
                                                    true,
                                                    String::from("Successfully signed in"),
                                                    None
                                                ));
                                        }
                                    }
                                }
                            }
                        }                        
                    } else {
                        return HttpResponse::Ok()
                                .json(ApiResponse::new(
                                    false,
                                    String::from("Username/password combination is incorrect"),
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


async fn sign_out_get() -> impl Responder {
    info!("sign_out_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn sign_out_post() -> impl Responder {
    info!("sign_out_post()");
    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}