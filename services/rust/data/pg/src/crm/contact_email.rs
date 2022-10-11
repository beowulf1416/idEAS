use log::{
    info,
    error,
    debug
};

use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager
};

use crate::{
    DbError,
    Dbo
};


pub struct ContactEmail(Dbo);

impl ContactEmail {
    pub fn new(client: Object<Manager>) -> Self {
        return Self(Dbo::new(client));
    }

    pub async fn add(
        &self,
        id: &uuid::Uuid,
        people_id: &uuid::Uuid,
        email: &str
    ) -> Result<(), DbError> {
        let t_email = crate::types::email::Email::new(email);
        return self.0.call_sp(
            "call crm.contact_email_add($1, $2, $3);",
            &[
                &id,
                &people_id,
                &t_email
            ]
        ).await;
    }

    pub async fn set_active(
        &self,
        id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call crm.contact_email_set_active($1, $2);",
            &[
                &id,
                &active
            ]
        ).await;
    }

    pub async fn verify(
        &self,
        id: &uuid::Uuid
    ) -> Result<(), DbError> {
        return self.0.call_sp(
            "call crm.contact_email_verified($1);",
            &[
                &id
            ]
        ).await;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use crate::Db;
}