use std::error::Error;

use postgres_types::{ 
    ToSql, 
    Type, 
    IsNull, 
    to_sql_checked 
};
use postgres_types::private::BytesMut;


#[derive(Debug)]
pub struct Email(String);


impl Email {
    pub fn new(email: &str) -> Self {
        return Self(String::from(email));
    }
}


impl ToSql for Email {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        return self.0.to_sql(ty, out);
    }

    to_sql_checked!();

    fn accepts(ty: &Type) -> bool {
        return ty.name() == "email_address";
    }
}