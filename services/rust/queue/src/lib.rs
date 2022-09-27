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
    },
    consumer::{
        Consumer, 
        FetchOffset, 
        GroupOffsetStorage
    }
};


use config::{
    ApplicationConfig,
    Provider,
    ProviderType
};


pub struct Queue {
    hosts: Vec<String>,
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

        let mut client = KafkaClient::new(hosts.to_owned());
        client.load_metadata_all().unwrap();

        // let producer = Producer::from_client(client)
        //     .create()
        //     .unwrap();

        return Self {
            hosts: hosts,
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
                    ProduceMessage::new(name, 0, None, Some(data.to_string().as_bytes()))
                )
            );
            debug!("response: {:?}", response);
        // }
    }

    pub fn create_consumer(
        &self,
        topic: &str
    ) -> Result<Consumer, bool> {
        match Consumer::from_hosts(self.hosts.to_owned())
            .with_topic_partitions(topic.to_owned(), &[0])
            .with_fallback_offset(FetchOffset::Earliest)
            .create() {
                Err(e) => {
                    error!("unable to create consumer: {:?}", e);
                    return Err(false);
                }
                Ok(consumer) => {
                    return Ok(consumer);
                }
            }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ctor::ctor;

    use serde_json::json;

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
            queue.send("test1", json!({ "test":"more tests", "test_1": "one more test" }));
            // debug!("topics: {:?}", topics);
        }  else {
            error!("unable to create queue");
            assert!(false);
        }
    }

    #[test]
    fn test_create_consumer() {
        if let Some(config) = config::get_configuration() {
            let queue = Queue::new(&config, "queue");
            let consumer = queue.create_consumer("test");
            // debug!("topics: {:?}", topics);
        }  else {
            error!("unable to create queue");
            assert!(false);
        }
    }
}
