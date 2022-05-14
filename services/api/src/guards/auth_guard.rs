use log::{ info, debug };

use actix_web::{
    guard::{ Guard, GuardContext }
};

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
                    let allow = permissions.contains(&self.permission);
                    debug!("AuthGuard::check() looking for: '{}' returning: {}", self.permission, allow);
                    return allow;
                }
            }

        debug!("AuthGuard::check() returning FALSE");
        return true;
    }
}