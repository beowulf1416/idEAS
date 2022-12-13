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


#[derive(Debug, Serialize, Deserialize)]
struct ItemRequest {
    pub id: uuid::Uuid,
    pub active: bool,
    pub name: String,
    pub description: String,
    
    pub sku: String,
    pub upc: String,

    pub is_perishable: bool,
    pub is_stocked: bool,
    pub is_purchased: bool,
    pub is_sold: bool,
    pub is_manufactured: bool
}


pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /clients
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(item_add_get))
                .route(web::post().to(item_add_post))
        )
        .service(
            web::resource("update")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(item_update_get))
                .route(web::post().to(item_update_post))
        )
    ;
}


async fn item_add_get() -> impl Responder {
    info!("item_add_get()");
    return HttpResponse::Ok().body("use POST method instead");
}

async fn item_add_post(
    db: web::Data<Db>,
    params: web::Json<ItemRequest>
) -> impl Responder {
    info!("item_add_post()");

    let error_message = String::from("an error occured while trying to process this request");

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            error_message,
            None
        ));
}

async fn item_update_get() -> impl Responder {
    info!("item_update_get()");
    return HttpResponse::Ok().body("use POST method instead");
}

async fn item_update_post(
    db: web::Data<Db>,
    params: web::Json<ItemRequest>
) -> impl Responder {
    info!("item_update_post()");

    let error_message = String::from("an error occured while trying to process this request");

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            error_message,
            None
        ));
}