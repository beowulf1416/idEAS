use log::{ info, debug, error };

use actix_web::{
    HttpRequest,
    guard::{ Guard, GuardContext }
};

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

        let extensions = ctx.req_data();
        if extensions.contains::<common::user::User>()
            && extensions.contains::<Vec<String>>() {
                if let Some(permissions) = extensions.get::<Vec<String>>() {
                    debug!("AuthGuard::check() permissions: {:?}", permissions);

                    // debug!("AuthGuard::check() returning TRUE");
                    // return true;

                    let allow = permissions.contains(&self.permission);
                    debug!("AuthGuard::check() looking for: '{}' returning: {}", self.permission, allow);
                    return allow;
                }
            }

        debug!("AuthGuard::check() returning FALSE");
        return true;

        // if let Some(header_value) = ctx.head().headers().get(header::AUTHORIZATION) {
        //     if let Ok(header_str) = header_value.to_str() {
        //         let token_value = String::from(header_str.replace("Bearer", "").trim());
        //         if !token_value.is_empty() {
        //             // TODO need to validate jwt token and retrieve claims
        //             // debug!("data/extensions: {:?}", ctx.req_data());
        //             let extensions = ctx.req_data();
        //             if extensions.contains::<common::user::User>() {
        //                 let user = extensions.get::<common::user::User>();
        //                 debug!("user: {:?}", user);
        //             }

        //             if extensions.contains::<String>() {
        //                 let permissions = extensions.get::<String>();
        //                 debug!("permissions: {:?}", permissions);
        //             }

        //             // let request = ctx
        //         } else {
        //             error!("empty token value");
        //         }
        //     } else {
        //         error!("unable to convert to string");
        //     }

        //     debug!("AuthGuard::check(): returning true");
        //     return true;
        // }

        // debug!("AuthGuard::check(): returning false");
        // return false;
    }
}