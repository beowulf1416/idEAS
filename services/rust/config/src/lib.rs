use std::{
    env,
    fs,
    sync::Once
};

use log::{
    info,
    error,
    debug
};

use serde::{
    Serialize,
    Deserialize
};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProviderType {
    #[serde(rename = "postgres")]
    Postgres
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub name: String,
    #[serde(rename = "type")]
    pub provider_type: ProviderType,
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


pub fn get_configuration() -> Option<ApplicationConfig> {
    if let Ok(cfg) = env::var("CFG") {
        match fs::read_to_string(cfg) {
            Err(e) => {
                error!("unable to read contents of file: {:?}", e);
                return None;
            }
            Ok(contents) => {
                let result: Result<ApplicationConfig, serde_json::Error>  = serde_json::from_str(&contents);
                match result {
                    Err(e) => {
                        error!("unable to parse contents: {:?}", e);
                        return None;
                    }
                    Ok(config) => {
                        return Some(config);
                    }
                }
            }
        }
    } else {
        error!("missing environment variable CFG");
        return None;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
