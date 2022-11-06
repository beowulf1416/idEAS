use log::{
    info,
    debug,
    error
};

use actix_web::guard::{
    Guard,
    GuardContext
};

use crate::extractors::user_parameter::UserParameter;

pub struct Permission {
    permission: String
}

impl Permission {
    pub fn new(permission: &str) -> Self {
        return Self {
            permission: String::from(permission)
        };
    }
}

impl Guard for Permission {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        // debug!("Permission::check(): {:?}", ctx);
        // debug!("{:?}", ctx.req_data().get::<UserParameter>());
        if let Some(up) = ctx.req_data().get::<UserParameter>() {
            let user = up.user();
            if let Some(permissions) = user.get_permissions() {
                let p = permissions.into_iter().find(|x| x.name == self.permission);
                // debug!("found: {:?}", p);
                return !p.is_none();
            }
        }
        return false;
    }
}