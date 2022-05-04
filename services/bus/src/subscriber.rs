use log::{ info, error };
// use std::fmt::Write;
use std::time::Duration;

use kafka::consumer::{ Consumer, FetchOffset, GroupOffsetStorage };

use serde_json::{ Value };


pub struct Subscriber {
    consumer: Consumer
}

impl Subscriber {

    pub fn new(
        hosts: Vec<String>, 
        topic: String, 
        group: String
    ) -> Self {
        let consumer = Consumer::from_hosts(hosts)
            .with_topic_partitions(topic, &[0, 1])
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group(group)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .unwrap();
        return Subscriber {
            consumer: consumer
        };
    }
}


#[cfg(test)]
mod tests {

    use std::sync::Once;
    static INIT: Once = Once::new();

    use serde_json::json;

    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }

    #[test]
    fn test_receive() {
        initialize();


    }
}