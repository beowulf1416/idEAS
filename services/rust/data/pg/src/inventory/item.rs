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


pub struct Item(Dbo);

impl Item {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn add(
        &self,
        client_id: &uuid::Uuid,
        item_id: &uuid::Uuid,
        name: &str,
        description: &str,
        sku: &str,
        upc: &str,
        dimension_id: &i32,
        uom_id: &i32,
        volume: &f32,
        weight: &f32,
        shelf_width: &f32,
        shelf_height: &f32,
        shelf_depth: &f32,
        perishable: &bool,
        stocked: &bool,
        purchased: &bool,
        sold: &bool,
        manufactured: &bool
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call inventory.item_add($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $16, $17, $18);",
            &[
                &client_id,
                &item_id,
                &name,
                &description,
                &sku,
                &upc,
                &dimension_id,
                &uom_id,
                &volume,
                &weight,
                &shelf_width,
                &shelf_height,
                &shelf_depth,
                &perishable,
                &stocked,
                &purchased,
                &sold,
                &manufactured
            ]
        ).await;
    }

    pub async fn update(
        &self,
        client_id: &uuid::Uuid,
        item_id: &uuid::Uuid,
        name: &str,
        description: &str,
        sku: &str,
        upc: &str,
        dimension_id: &i32,
        uom_id: &i32,
        volume: &f32,
        weight: &f32,
        shelf_width: &f32,
        shelf_height: &f32,
        shelf_depth: &f32,
        perishable: &bool,
        stocked: &bool,
        purchased: &bool,
        sold: &bool,
        manufactured: &bool
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call inventory.item_update($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $16, $17, $18);",
            &[
                &client_id,
                &item_id,
                &name,
                &description,
                &sku,
                &upc,
                &dimension_id,
                &uom_id,
                &volume,
                &weight,
                &shelf_width,
                &shelf_height,
                &shelf_depth,
                &perishable,
                &stocked,
                &purchased,
                &sold,
                &manufactured
            ]
        ).await;
    }

    pub async fn set_active(
        &self,
        item_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call inventory.item_set_active($1, $2);",
            &[
                &item_id,
                &active
            ]
        ).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use crate::Db;


    #[actix_rt::test]
    async fn test_item_add() {
        if let Some(config) = config::get_configuration() {
            let db = Db::new(&config);
            match db.get_client().await {
                Err(e) => {
                    error!("unable to retrieve client {:?}", e);
                    assert!(false);
                }
                Ok(client) => {
                    let item = Item::new(client);

                    let item_id = uuid::Uuid::new_v4();

                    let mut rng = rand::thread_rng();
                    let suffix: u8 = rng.gen();

                    
                }
            }
        } else {
            assert!(false);
        }
    }
}