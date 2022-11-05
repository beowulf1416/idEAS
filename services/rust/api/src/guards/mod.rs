pub mod permission;

use permission::Permission;


pub fn Permission(permission: &str) -> permission::Permission {
    return permission::Permission::new(permission);
}