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


pub struct People {
    client: Object<Manager>
}


impl People {

    pub fn new(client: Object<Manager>) -> Self {
        return Self {
            client: client
        };
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use crate::Db;
}