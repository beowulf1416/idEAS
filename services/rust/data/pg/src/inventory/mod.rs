pub mod item;
pub mod item_balances;


use log::{
    info,
    error,
    debug
};


use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager
};
// use tokio_postgres::{
//     error::SqlState
// };

use crate::{
    DbError,
    Dbo
};


pub struct Inventory {
    dbo: Dbo,
    item: Option<item::Item>
}


impl Inventory {
    pub fn new(client: Object<Manager>) -> Self {
        return Self {
            dbo: Dbo::new(client),
            item: None
        };
    }
}