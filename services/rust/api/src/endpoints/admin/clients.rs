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
    DbError
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



pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /clients
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(client_add_get))
                .route(web::post().to(client_add_post))
        )
        // .service(
        //     web::resource("members")
        //         .route(web::method(http::Method::OPTIONS).to(default_options))
        //         .route(web::get().to(client_members_get))
        //         .route(web::post().to(client_members_post))
        // )
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

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            String::from("Service is up. version: 1.0.0.0.dev"),
            None
        ));
}


// async fn client_members_get() -> impl Responder {
//     info!("client_members_get()");
//     return HttpResponse::Ok().body("use POST method instead");
// }


// async fn client_members_post(
//     db: web::Data<Db>
// ) -> impl Responder {
//     info!("client_members_post()");

//     return HttpResponse::Ok()
//         .json(ApiResponse::new(
//             false,
//             String::from("Service is up. version: 1.0.0.0.dev"),
//             None
//         ));
// }