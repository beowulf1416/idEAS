pub mod types;

pub mod countries;
pub mod currencies;
pub mod uom;
pub mod client;
pub mod organization;
pub mod auth;
pub mod user;
pub mod users;

pub mod iam;
pub mod people;
pub mod crm;
pub mod inventory;


use log::{
    info,
    error,
    debug
};

use std::str::FromStr;

use postgres_types::{ 
    ToSql
};
use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager, 
    ManagerConfig, 
    Pool, 
    RecyclingMethod 
};
use tokio_postgres::{
    NoTls,
    // row::Row
};
use tokio_postgres::config::{ Config };

use actix_web:: {
    dev::Payload,
    http::StatusCode,
    HttpRequest, 
    // HttpMessage,
    FromRequest,
    ResponseError,
    // web
};

use futures::{
    future::{
        ok,
        err,
        Ready
    }
};


use config::{
    ApplicationConfig,
    ProviderType
};


#[derive(Debug, PartialEq)]
pub enum DbError {
    ClientError,
    DuplicateKeyError
}

impl ResponseError for DbError {

    fn status_code(&self) -> StatusCode {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
}

impl std::fmt::Display for DbError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DbError::ClientError => write!(f, "internal server error"),
            _ => todo!()
        }
    }
}


#[derive(Clone)]
pub struct Db {
    pool: Option<Pool>
}


impl Db {

    pub fn new(cfg: &ApplicationConfig) -> Self {
        for p in cfg.providers.clone() {
            if matches!(p.provider_type, ProviderType::Postgres) {
                debug!("provider: {:?}", p);
                for url in p.url {
                    match Config::from_str(&url) {
                        Err(e) => {
                            error!("unable to parse database connection string '{}' {:?}", url, e);
                        }
                        Ok(db_cfg) => {
                            let mgr = Manager::from_config(
                                db_cfg,
                                NoTls,
                                ManagerConfig {
                                    recycling_method: RecyclingMethod::Fast
                                }
                            );
    
                            match Pool::builder(mgr)
                                .max_size(16)
                                .build() {
                                    Err(e) => {
                                        error!("unable to create database pool: {:?}", e);
                                    }
                                    Ok(pool) => {
                                        return Self {
                                            pool: Some(pool)
                                        };
                                    }
                                }
                        }
                    }
                }
            }
        }

        return Self {
            pool: None
        };
    }

    pub async fn get_client(&self) -> Result<Object<Manager>, DbError> {
        if let Some(pool) = &self.pool {
            match pool.get().await {
                Err(e) => {
                    error!("unable to retrieve client from pool {:?}", e);
                    return Err(DbError::ClientError);
                }
                Ok(client) => {
                    return Ok(client);
                }
            }
        } else {
            return Err(DbError::ClientError);
        }
    }
}


//https://stackoverflow.com/questions/63308246/how-to-use-async-code-in-actix-web-extractors
impl FromRequest for Db {
    type Error = DbError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(db) = request.app_data::<Self>() {
            return ok(db.clone());
        }
        return err(DbError::ClientError);
    }
}



pub struct Dbo {
    client: Object<Manager>
}


impl Dbo {

    pub fn new(client: Object<Manager>) -> Self {
        return Self {
            client: client
        };
    }

    pub fn get_client(
        &self
    ) -> &Object<Manager> {
        return &self.client;
    }

    // pub fn get_stmt(
    //     &self,
    //     sql: &sql
    // ) -> Result<Statement, DbError> {

    // }


    pub async fn call_sp(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)]
    ) -> Result<(), DbError> {
        match &self.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare query: {} {:?}", sql, e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match &self.client.execute(
                    stmt,
                    &params
                ).await {
                    Err(e) => {
                        error!("unable to execute query: {} {:?}", sql, e);
                        return Err(DbError::ClientError);
                    }
                    Ok(_) => {
                        return Ok(());
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // use std::env;
    use ctor::ctor;

    // use std::sync::Once;
    // static INIT: Once = Once::new();


    #[ctor]
    fn initialize() {
        // INIT.call_once( || {
            env_logger::init();
        // });
    }

    #[actix_rt::test] 
    async fn test_db_new() {
        // initialize();

        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            if let Err(e) = db.get_client().await {
                error!("unable to retrieve client: {:?}", e);
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
}
