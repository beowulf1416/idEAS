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

// use common::{
//     dimension::Dimension,
//     uom::UOM
// };



pub struct Dimension(Dbo);

impl Dimension {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn fetch(
        &self
    ) -> Result<Vec<common::dimension::Dimension>, DbError> {
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
                        let results = rows.iter().map(|r| common::dimension::Dimension {
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
        &self,
        dimension: &i16
    ) -> Result<Vec<common::uom::UOM>, DbError> {
        let sql = "select * from common.uom_fetch($1);";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                    &stmt,
                    &[
                        &dimension
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records: {:?}", e);
                        return Err(DbError::ClientError);
                    }
                    Ok(rows) => {
                        debug!("uom rows: {:?}", rows);
                        let results = rows.iter().map(|r| common::uom::UOM {
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

    pub async fn dimensions(
        &self
    ) -> Result<Vec<common::dimension::Dimension>, DbError> {
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
                        debug!("dimension rows: {:?}", rows);
                        let results = rows.iter().map(|r| common::dimension::Dimension {
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


#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use crate::Db;

    #[actix_rt::test]
    async fn test_dimensions_fetch() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let uom = UOM::new(client);
                    match uom.dimensions().await {
                        Err(e) => {
                            error!("unable to fetch dimensions: {:?}", e);
                            assert!(false);
                        }
                        Ok(dimensions) => {
                            debug!("dimensions: {:?}", dimensions);
                            assert!(true);
                        }
                    }
                }
            }
        } else {
            error!("unable to get configuration");
            assert!(false);
        }
    }

    #[actix_rt::test]
    async fn test_uom_fetch() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client: {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let uom = UOM::new(client);
                    match uom.fetch(&1).await {
                        Err(e) => {
                            error!("unable to fetch uom: {:?}", e);
                            assert!(false);
                        }
                        Ok(uoms) => {
                            debug!("uom: {:?}", uoms);
                            assert!(true);
                        }
                    }
                }
            }
        } else {
            error!("unable to get configuration");
            assert!(false);
        }
    }
}