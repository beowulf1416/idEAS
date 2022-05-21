pub enum Permissions {
    UserCurrent,
    TenantAdd,
    TenantSetActive
}

impl Permissions {

    pub fn as_str(&self) -> String {
        match self {
            Permissions::UserCurrent => String::from("user.current"),
            Permissions::TenantAdd => String::from("tenant.add"),
            Permissions::TenantSetActive => String::from("tenant.active")
        }
    }
}