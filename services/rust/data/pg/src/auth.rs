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

use crate::DbError;


pub struct Auth {
    client: Object<Manager>
}


impl Auth {

    pub fn new(client: Object<Manager>) -> Self {
        return Self {
            client: client
        };
    }


    pub fn register(
        &self,
        email: &str
    ) -> Result<(), DbError> {
        return Ok(());
    }
}