pub mod auth;


use log::{
    info,
    error,
    debug
};

use std::str::FromStr;

use deadpool_postgres::{ 
    Manager, 
    ManagerConfig, 
    Pool, 
    RecyclingMethod 
};
use tokio_postgres::NoTls;
use tokio_postgres::config::{ Config };

use actix_web:: {
    dev::Payload,
    http::StatusCode,
    HttpRequest, 
    HttpMessage,
    FromRequest,
    ResponseError,
    web
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


#[cfg(test)]
mod tests {
    use super::*;

}
