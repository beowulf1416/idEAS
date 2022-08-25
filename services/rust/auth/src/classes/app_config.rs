use serde::{
    Serialize,
    Deserialize
};


#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub url: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationConfig {
    #[serde(rename = "bindIp")]
    pub bind_ip: String,
    
    #[serde(rename = "bindPort")]
    pub bind_port: i32,
    pub providers: Vec<Provider>
}