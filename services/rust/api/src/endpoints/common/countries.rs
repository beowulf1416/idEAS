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
    countries::Countries
};

use crate::endpoints::{
    ApiResponse,
    default_options
};


#[derive(Debug, Serialize, Deserialize)]
struct CountriesFetchRequest {
    // filter: String,
    // items: i32,
    // page: i32
}


#[derive(Debug, Serialize, Deserialize)]
struct CountryGet {
    id: i32
}


pub fn config(cfg: &mut web::ServiceConfig) {
    /// base url /countries
    cfg
        .service(
            web::resource("fetch")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(countries_fetch_get))
                .route(web::post().to(countries_fetch_post))
        )
        .service(
            web::resource("get")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(countries_get_get))
                .route(web::post().to(countries_get_post))
        )
    ;
}


async fn countries_fetch_get() -> impl Responder {
    info!("countries_fetch_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn countries_fetch_post(
    db: web::Data<Db>,
    params: web::Json<CountriesFetchRequest>
) -> impl Responder {
    info!("countries_fetch_post()");

    match db.get_client().await {
        Err(e) => {
            error!("unable to retrieve client");
        }
        Ok(client) => {
            let countries_dbo = Countries::new(client);
            match countries_dbo.fetch().await {
                Err(e) => {
                    error!("unable to fetch countries");
                }
                Ok(result) => {
                    // debug!("result: {:?}", result);
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            String::from("successfully retrieved countries"),
                            Some(json!({
                                "countries": result
                            }))
                        ))
                }
            }
        }
    }

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from("An error occured while retrieving countries"),
            None
        ));
}


async fn countries_get_get() -> impl Responder {
    info!("countries_get_get()");
    return HttpResponse::Ok().body("use POST method instead");
}


async fn countries_get_post(
    db: web::Data<Db>,
    params: web::Json<CountriesFetchRequest>
) -> impl Responder {
    info!("countries_get_post()");

    

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            String::from("An error occured while retrieving country information"),
            None
        ));
}