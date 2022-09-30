extern crate log;

use std::str;

use log::{
    info,
    debug,
    error
};

use serde_json::Value;

use kafka::{
    consumer::{
        Consumer, 
        FetchOffset, 
        GroupOffsetStorage
    }
};

use config::{
    ProviderType,
    get_configuration
};


const QUEUE_NAME: &str = "queue";
const TOPIC: &str = "mailer";
const GROUP: &str = "mailer";


fn main() {
    env_logger::init();


    println!("Hello, world!");

    if let Some(cfg) = get_configuration() {
        let hosts: Vec<String> = cfg.providers.iter()
            .filter(|x| matches!(x.provider_type, ProviderType::Kafka) && x.name == QUEUE_NAME)
            .map(|r| r.url.clone())
            .flatten()
            .collect();
        
        match Consumer::from_hosts(hosts.to_owned())
            .with_topic(TOPIC.to_owned())
            .with_group(GROUP.to_owned())
            .with_fallback_offset(FetchOffset::Earliest)
            .create() {
                Err(e) => {
                    error!("unable to create consumer: {:?}", e);
                }
                Ok(mut consumer) => {
                    loop {
                        for ms in consumer.poll().unwrap().iter() {
                            for m in ms.messages() {
                                // debug!("message: {:?}", m);
        
                                if let Ok(sz) = str::from_utf8(m.value) {
                                    let v: Value = serde_json::from_str(sz).unwrap();
                                    debug!("message json: {:?}", v);
                                } else {
                                    debug!("unknown message: {:?}", m);
                                }
                            }
                            consumer.consume_messageset(ms);
                        }
                        consumer.commit_consumed().unwrap();
                    }
                }
            }
    } else {
        error!("unable to retrieve configuration");
    }
}
