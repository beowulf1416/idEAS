use log::{
    info,
    error,
    debug
};


use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager
};
use tokio_postgres::{
    error::SqlState
};

use crate::{
    DbError,
    Dbo
};


pub struct UserDbo(Dbo);

impl UserDbo {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn add(
        &self,
        email: &str,
        password: &str
    ) -> Result<(), DbError> {
        info!("UserClient::add()");
        let t_email = crate::types::email::Email::new(email);
        return self.0.call_sp(
            "call iam.user_add($1, $2);",
            &[
                &t_email,
                &password
            ]
        ).await;
    }
}