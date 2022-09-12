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
    country::Country
};


pub struct Countries(Dbo);

impl Countries {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn fetch(
        &self
    ) -> Result<Vec<Country>, DbError> {
        let sql = "select * from common.countries_fetch();";
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
                        let results = rows.iter().map(|r| Country {
                            id: r.get("id"),
                            name: r.get("name"),
                            alpha_2: r.get("iso_3166_1_alpha_2"),
                            alpha_3: r.get("iso_3166_1_alpha_3")
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
    async fn test_countries_fetch() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let countries = Countries::new(client);
                    match countries.fetch().await {
                        Err(e) => {
                            error!("unable to fetch countries: {:?}", e);
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