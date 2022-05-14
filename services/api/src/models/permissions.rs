pub enum Permissions {
    UserCurrent
}

impl Permissions {

    pub fn as_str(&self) -> String {
        match self {
            Permissions::UserCurrent => String::from("user.current")
        }
    }
}