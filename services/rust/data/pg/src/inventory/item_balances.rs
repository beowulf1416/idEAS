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


pub struct ItemBalances(Dbo);

impl ItemBalances {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn add(
        &self,
        client_id: &uuid::Uuid,
        item_id: &uuid::Uuid,
        quantity: &f32
    ) -> Result<(), DbError>{
        return self.0.call_sp(
            "call inventory.item_balance_add($1, $2, $3);",
            &[
                &client_id,
                &item_id,
                &quantity
            ]
        ).await;
    }
}