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


pub struct Role(Dbo);

impl Role {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn add(
        &self,
        id: &uuid::Uuid,
        client_id: &uuid::Uuid,
        name: &str,
        description: &str
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call iam.role_add($1, $2, $3, $4);",
            &[
                &id,
                &client_id,
                &name,
                &description
            ]
        ).await;
    }

    pub async fn update(
        &self,
        id: &uuid::Uuid,
        client_id: &uuid::Uuid,
        name: &str,
        description: &str
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call iam.role_update($1, $2, $3, $4);",
            &[
                &id,
                &client_id,
                &name,
                &description
            ]
        ).await;
    }

    pub async fn set_active(
        &self,
        id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call iam.role_set_active($1, $2);",
            &[
                &id,
                &active
            ]
        ).await;
    }

    pub async fn fetch(
        &self,
        client_id: &uuid::Uuid,
        filter: &str,
        items: &i32,
        page: &i32
    ) -> Result<Vec<common::iam::role::Role>, DbError> {
        let sql = "select * from iam.role_fetch($1, $2, $3, $4)";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                        &stmt,
                        &[
                            &client_id,
                            &filter,
                            &items,
                            &page
                        ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results: Vec<common::iam::role::Role> = rows.iter().map(|r| common::iam::role::Role {
                            id: r.get("id"),
                            active: r.get("active"),
                            name: r.get("name"),
                            description: r.get("description")
                        })
                        .collect();
                        return Ok(results);
                    }
                }
            }
        }
    }

    pub async fn get(
        &self,
        role_id: &uuid::Uuid
    ) -> Result<common::iam::role::Role, DbError> {
        let sql = "select * from iam.role_get($1)";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query_one(
                        &stmt,
                        &[
                            &role_id
                        ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(row) => {
                        let result: common::iam::role::Role = common::iam::role::Role {
                            id: row.get("id"),
                            active: row.get("active"),
                            name: row.get("name"),
                            description: row.get("description")
                        };
                        return Ok(result);
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use crate::Db;


}