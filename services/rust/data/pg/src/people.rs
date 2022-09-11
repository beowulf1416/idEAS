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


pub struct People(Dbo);


impl People {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use crate::Db;
}