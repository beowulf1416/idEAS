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
                            name: r.get("name")
                        })
                        .collect();
                        return Ok(results);
                    }
                }
            }
        }
    }

    pub async fn filter(
        &self,
        filter: &str,
        active: &bool,
        items: &i32,
        page: &i32
    ) -> Result<Vec<common::iam::permission::Permission>, DbError> {
        let sql = "select * from iam.permission_filter($1, $2, $3);";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare sql: {:?}", e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                    &stmt,
                    &[
                        &filter,
                        &items,
                        &page
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve permissions: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| common::iam::permission::Permission {
                            name: r.get("name")
                        })
                        .collect();
                        return Ok(results);
                    }
                }
            }
        }
    }

    pub async fn get_role_permissions(
        &self,
        role_id: &uuid::Uuid
    ) -> Result<Vec<common::iam::permission::Permission>, DbError> {
        let sql = "select * from iam.role_permissions_get($1);";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare sql: {:?}", e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                    &stmt,
                    &[
                        &role_id
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve permissions: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| common::iam::permission::Permission {
                            name: r.get("name")
                        })
                        .collect();
                        return Ok(results);
                    }
                }
            }
        }
    }

    pub async fn get_role_permissions_not_assigned(
        &self,
        role_id: &uuid::Uuid
    ) -> Result<Vec<common::iam::permission::Permission>, DbError> {
        let sql = "select * from iam.role_permissions_not_assigned($1);";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare sql: {:?}", e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                    &stmt,
                    &[
                        &role_id
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve permissions: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| common::iam::permission::Permission {
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