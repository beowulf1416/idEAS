use log::{ info };

use actix_web::guard::{ Guard, GuardContext };
use http::header;

pub struct AuthGuard {}


impl AuthGuard {
    pub fn new() -> Self {
        return AuthGuard {};
    }
}

impl Guard for AuthGuard {

    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        info!("AuthGuard::check(): {:?}", ctx); 
        if let Some(_) = ctx.head().headers().get(header::AUTHORIZATION) {
            return true;
        }
        return false;
    }
}