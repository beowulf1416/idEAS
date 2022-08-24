use log::{ info, error };
// use std::fmt::Write;
use std::time::Duration;

use kafka::producer::{ Producer, Record, RequiredAcks };

use serde_json::{ Value };



// #[derive(Clone)]
pub struct Publisher {
    producer: Producer
}


impl Publisher {

    pub fn new(hosts: Vec<String>) -> Self {
        let producer = Producer::from_hosts(hosts)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();
        return Publisher {
            producer: producer
        };
    }

    pub fn send(&mut self, key: &str, json: Value) -> Result<(), String> {
        info!("Publisher::send()");

        if let Ok(_) = self.producer.send(&Record::from_value(key, json.to_string().as_bytes())) {
            return Ok(());
        }

        error!("unable to send value");
        return Err(String::from("unable to publish value"));
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Once;
    static INIT: Once = Once::new();

    use serde_json::json;
    use crate::publisher::Publisher;

    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }

    #[test]
    fn test_send() {
        initialize();
        
        let mut publisher = Publisher::new(vec!(String::from("127.0.0.1:9092")));
        // publisher.send("test", json!({ "key": "value" }));
        if let Err(e) = publisher.send("my.test", json!({ "key": "value" })) {
            assert!(false);
        }
    }

}