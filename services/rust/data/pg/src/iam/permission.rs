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

    pub async fn get_user_permissions(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<Vec<common::iam::permission::Permission>, DbError> {
        let sql = "select * from iam.user_permissions_fetch($1);";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare sql: {:?}", e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                    &stmt,
                    &[
                        &user_id
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve user permissions: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| common::iam::permission::Permission {
                            id: r.get("id"),
                            name: r.get("name")
                        })
                        .collect();
                        return Ok(results);
                    }
                }
            }
        }
    }
}