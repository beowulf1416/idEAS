pub mod permission;

use permission::Permission;


pub fn permission(permission: &str) -> permission::Permission {
    return permission::Permission::new(permission);
}