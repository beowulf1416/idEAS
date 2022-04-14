/**
 * common endpoint handlers
 */
use log::{ info };

use actix_web::{ HttpResponse, Responder };


/// default OPTIONS response
pub async fn default_options() -> impl Responder {
    info!("endpoints::common::default_options()");
    return HttpResponse::Ok()
        .finish();
}