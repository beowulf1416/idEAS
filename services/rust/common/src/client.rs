#[derive(Debug)]
pub struct Client {
    pub id: String,
    pub active: bool,
    pub name: String,
    pub description: String,
    pub address: String,
    pub country_id: i32,
    pub url: String
}