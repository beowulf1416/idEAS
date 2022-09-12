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
    currency::Currency
};


pub struct Currencies(Dbo);

impl Currencies {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn fetch(
        &self
    ) -> Result<Vec<Currency>, DbError> {
        let sql = "select * from common.currencies_fetch();";
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
                        let results = rows.iter().map(|r| Currency {
                            id: r.get("id"),
                            name: r.get("name"),
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


#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use crate::Db;

    #[actix_rt::test]
    async fn test_currencies_fetch() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let currencies = Currencies::new(client);
                    match currencies.fetch().await {
                        Err(e) => {
                            error!("unable to fetch currencies: {:?}", e);
                            assert!(false);
                        }
                        Ok(records) => {
                            // debug!("records: {:?}", records);
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