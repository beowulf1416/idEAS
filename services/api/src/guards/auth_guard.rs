use log::{ info, debug };

use actix_web::{
    guard::{ Guard, GuardContext }
};

use crate::models::permissions::Permissions;



pub struct AuthGuard {
    permission: Permissions
}


impl AuthGuard {
    pub fn new(
        permission: Permissions
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
                    let permission = self.permission.as_str();
                    let allow = permissions.contains(&permission);
                    debug!("AuthGuard::check() looking for: '{}' returning: {}", permission, allow);
                    return allow;
                }
            }

        debug!("AuthGuard::check() returning FALSE");
        return true;
    }
}