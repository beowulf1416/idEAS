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


pub struct UserClient(Dbo);

impl UserClient {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn add(
        &self,
        user_id: &uuid::Uuid,
        client_id: &uuid::Uuid
    ) -> Result<(), DbError> {
        info!("UserClient::add()");
        return self.0.call_sp(
            "call iam.user_client_add($1, $2);",
            &[
                &user_id,
                &client_id
            ]
        ).await;
    }

    pub async fn fetch(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<Vec<common::client::Client>, DbError> {
        let sql = "select * from iam.user_client_fetch($1)";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
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
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| common::client::Client {
                            id: r.get("id"),
                            active: r.get("active"),
                            name: r.get("name"),
                            description: r.get("description"),
                            address: r.get("address"),
                            country_id: r.get("country_id"),
                            url: r.get("url")
                        })
                        .collect();
                        return Ok(results);
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

    lazy_static!{
        static ref t_user_id: uuid::Uuid = {
            return uuid::Uuid::new_v4();
        };
    }
}