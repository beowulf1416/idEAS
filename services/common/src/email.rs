/// Email Address
// use std::error::Error;

use serde::{ Serialize, Deserialize };

// use postgres_types::{ ToSql, Type, IsNull, to_sql_checked };
// use postgres_types::private::BytesMut;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Email(String);


impl Email {

    pub fn new(email: String) -> Result<Self, String> {
        if email.is_empty() {
            return Err(String::from("email address cannot be empty"));
        } else {
            return Ok(Email(email));
        }
    }

    pub fn get_email_str(&self) -> String {
        return self.0.clone();
    }
}

// impl ToSql for Email {

//     fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
//         return self.email.to_sql(ty, out);
//     }

//     to_sql_checked!();

//     fn accepts(ty: &Type) -> bool {
//         return ty.name() == "email";
//     }
// }