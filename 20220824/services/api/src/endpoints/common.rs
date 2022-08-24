/**
 * common endpoint handlers
 */
use log::{ info };

use actix_web::{ HttpResponse, Responder };

// use users::user_param::UserParam;


/// default OPTIONS response
pub async fn default_options() -> impl Responder {
    info!("endpoints::common::default_options()");
    return HttpResponse::Ok()
        .finish();
}

// pub async fn options_permissions(
//     _user_param: UserParam
// ) -> impl Responder {

//     return HttpResponse::Ok()
//         .finish();
// }