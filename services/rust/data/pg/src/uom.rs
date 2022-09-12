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

use common::{
    dimension::Dimension,
    uom::UOM
};



pub struct Dimension(Dbo);

impl Dimension {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn fetch(
        &self
    ) -> Result<Vec<Dimension>, DbError> {
        let sql = "select * from common.dimensions_fetch();";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                    &stmt,
                    &[]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| Dimension {
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


pub struct UOM(Dbo);

impl UOM {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn fetch(
        &self
    ) -> Result<Vec<UOM>, DbError> {
        let sql = "select * from common.uom_fetch();";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                    &stmt,
                    &[]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        let results = rows.iter().map(|r| UOM {
                            id: r.get("id"),
                            name: r.get("name"),
                            description: r.get("description"),
                            symbol: r.get("symbol")
                        })
                        .collect();
                        return Ok(results);
                    }
                }
            }
        }
    }
}