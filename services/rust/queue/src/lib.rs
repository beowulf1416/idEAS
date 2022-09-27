use log::{
    info,
    error,
    debug
};

use std::time::Duration;

use serde_json::Value;

use kafka::{
    client::{
        KafkaClient,
        ProduceMessage,
        RequiredAcks
    },
    producer::{
        Producer
    }
};


use config::{
    ApplicationConfig,
    Provider,
    ProviderType
};


pub struct Queue {
    client: KafkaClient,
    // producer: Producer
}

impl Queue {

    pub fn new(cfg: &ApplicationConfig, name: &str) -> Self {
        let hosts: Vec<String> = cfg.providers.iter()
            .filter(|x| matches!(x.provider_type, ProviderType::Kafka) && x.name == name)
            .map(|r| r.url.clone())
            .flatten()
            .collect();

        debug!("hosts: {:?}", hosts);

        let client = KafkaClient::new(hosts);
        // client.load_metadata_all().unwrap();

        // let producer = Producer::from_client(client)
        //     .create()
        //     .unwrap();

        return Self {
            client: client,
            // producer: producer
        };
    }

    // pub fn topics(&mut self) {
    //     self.client.load_metadata_all().unwrap();

    //     let topic_names = self.client.topics().names();
    //     let names: Vec<&str> = topic_names.collect();

    //     debug!("topics: {:?}", names);

    //     // match self.client.load_metadata_all() {
    //     //     Err(e) => {
    //     //         error!("unable to fetch metadata: {:?}", e);
    //     //     }
    //     //     Ok(r) => {
    //     //         debug!("result: {:?}", r);
    //     //     }
    //     // }
    // }


    pub fn send(
        &mut self,
        name: &str,
        data: Value
    ) {
        // if self.client.topics().contains(&name) {
            let response = self.client.produce_messages(
                RequiredAcks::One,
                Duration::from_millis(1000),
                vec!(
                    // ProduceMessage::new(name, 0, None, Some("test".as_bytes()))
                    ProduceMessage::new(name, 0, None, Some(data.as_bytes()))
                )
            );
            debug!("response: {:?}", response);
        // }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ctor::ctor;

    #[ctor]
    fn initialize() {
        env_logger::init();
    }

    #[test]
    fn test_new() {
        if let Some(config) = config::get_configuration() {
            let queue = Queue::new(&config, "queue");
        }  else {
            error!("unable to create queue");
            assert!(false);
        }
    }

    #[test]
    fn test_send() {
        if let Some(config) = config::get_configuration() {
            let mut queue = Queue::new(&config, "queue");
            queue.send("test");
            // debug!("topics: {:?}", topics);
        }  else {
            error!("unable to create queue");
            assert!(false);
        }
    }
}
