use log::{
    info,
    error,
    debug
};


use actix_web::{ web };

use config::{
    ApplicationConfig,
    ProviderType
};

use pg::Db;



pub struct Data {
    db: Db
}


impl Data {

    pub fn new(cfg: &ApplicationConfig) -> Self {
        let db = Db::new(cfg);
        return Data {
            db: db
        };
    }

    pub fn get_db(&self) -> Db {
        return self.db.clone();
    }
}