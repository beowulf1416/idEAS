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


pub struct UsersDbo(Dbo);

impl UsersDbo {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub fn fetch(
        &self,
        filter: &str,
        items: &i32,
        page: &i32
    ) {
        debug!("UsersDbo::fetch() {:?}", email);

        let sql = "select * from iam.user_get_by_email($1);";
        match self.0.client.prepare_cached(sql).await {
            Err(e) => {
                error!("unable to prepare sql: {:?}", e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.0.client.query(
                    &stmt,
                    &[
                        &filter,
                        &items,
                        &page
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve records");
                    }
                    Ok(rows) => {
                        let users = rows.iter().map(|r| common::iam::user::User {
                            id: r.get('id'),
                            active: r.get('active'),
                            email: r.get('email')
                        })
                        .collect()
                        return users;
                    }
                }
            }
        }
    }
}