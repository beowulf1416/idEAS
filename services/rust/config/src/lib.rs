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
    Postgres,

    #[serde(rename = "kafka")]
    Kafka
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub name: String,
    #[serde(rename = "type")]
    pub provider_type: ProviderType,
    pub url: Vec<String>
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Auth {
//     #[serde(rename = "bindHost")]
//     pub bind_host: String,
    
//     #[serde(rename = "bindPort")]
//     pub bind_port: i32,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Api {
    #[serde(rename = "bindHost")]
    pub bind_host: String,
    
    #[serde(rename = "bindPort")]
    pub bind_port: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mailer {
    pub host: String,
    pub user: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub secret: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationConfig { 
    pub base_url: String,
    // pub auth: Auth,
    pub api: Api,
    pub mailer: Mailer,
    pub token: Token,

    pub providers: Vec<Provider>
}


pub fn get_configuration() -> Option<ApplicationConfig> {
    if let Ok(cfg) = env::var("CFG") {
        info!("using config file: {}", cfg);
        // info!("absolute path: {}", fs::canonicalize(cfg.clone()).unwrap().display());
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

    use std::env;

    use std::sync::Once;
    static INIT: Once = Once::new();

    use super::*;


    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }

    #[test]
    fn test_configuration() {
        initialize();

        match get_configuration() {
            None => {
                error!("configuration file not found");
                assert!(false);
            }
            Some(config) => {
                debug!("configuration loaded: {:?}", config);
                assert!(true);
            }
        }
    }
}
