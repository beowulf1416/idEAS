use serde::{
    Serialize,
    Deserialize
};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub name: String,
    pub url: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationConfig {
    #[serde(rename = "bindHost")]
    pub bind_host: String,
    
    #[serde(rename = "bindPort")]
    pub bind_port: i32,
    pub providers: Vec<Provider>
}