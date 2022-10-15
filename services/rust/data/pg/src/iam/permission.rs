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


pub struct Permissions(Dbo);

impl Permissions {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn get_all(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<Vec<String>, DbError> {
        let sql = "select * from iam.user_permissions_fetch($1);";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare sql: {:?}", e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                let t_email = crate::types::email::Email::new(email);
                match self.0.client.query_one(
                    &stmt,
                    &[
                        &user_id
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve user permissions: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(r) => {
                        
                        return Ok(common::user::User::new(
                            Some(r.get("id")),
                            r.get("active"),
                            r.get("email")
                        ));
                    }
                }
            }
        }
    }
}