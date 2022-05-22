pub enum Permissions {
    UserCurrent,
    TenantAdd,
    TenantSetActive,
    TenantsList
}

impl Permissions {

    pub fn as_str(&self) -> String {
        match self {
            Permissions::UserCurrent => String::from("user.current"),
            Permissions::TenantAdd => String::from("tenant.add"),
            Permissions::TenantSetActive => String::from("tenant.active"),
            Permissions::TenantsList => String::from("tenants.list")
        }
    }
}