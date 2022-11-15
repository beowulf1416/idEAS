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
        user_id: &uuid::Uuid,
        email: &str,
        password: &str
    ) -> Result<(), DbError> {
        info!("UserDbo::add()");
        let t_email = crate::types::email::Email::new(email);
        return self.0.call_sp(
            "call iam.user_signup($1, $2, $3);",
            &[
                &user_id,
                &t_email,
                &password
            ]
        ).await;
    }

    pub async fn get(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<common::iam::user::User, DbError> {
        info!("UserDbo::get()");
        let sql = "select * from iam.user_get($1)";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query_one(
                        &stmt,
                        &[
                            &user_id
                        ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(r) => {
                        let result = common::iam::user::User {
                            id: r.get("id"),
                            active: r.get("active"),
                            email: r.get("email")
                        };
                        return Ok(result);
                    }
                }
            }
        }
    }
}