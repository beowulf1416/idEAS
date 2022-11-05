use log::{
    info,
    debug,
    error
};

use actix_web::guard::{
    Guard,
    GuardContext
};


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
        debug!("Permission::check(): {:?}", ctx);
        return true;
    }
}