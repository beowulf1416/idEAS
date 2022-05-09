use log::{ info, debug, error };

use actix_web::guard::{ Guard, GuardContext };
use http::header;

pub struct AuthGuard {
    permission: String
}


impl AuthGuard {
    pub fn new(
        permission: String
    ) -> Self {
        return AuthGuard {
            permission: permission
        };
    }
}

impl Guard for AuthGuard {

    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        info!("AuthGuard::check(): {:?}", ctx); 
        if let Some(header_value) = ctx.head().headers().get(header::AUTHORIZATION) {
            if let Ok(header_str) = header_value.to_str() {
                let token_value = header_str.replace("Bearer", "");
                if !token_value.is_empty() {
                    // TODO need to validate jwt token and retrieve claims
                } else {
                    error!("unable retrieve token value");
                }
            } else {
                error!("unable to convert to string");
            }
            debug!("header value: {:?}", header_value);

            debug!("AuthGuard::check(): returning true");
            return true;
        }

        debug!("AuthGuard::check(): returning false");
        return false;
    }
}