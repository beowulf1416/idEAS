extern crate log;

use log::{ info, error };

use std::fmt::Write;
use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};

fn main() {
    env_logger::init();
    info!("starting up ...");
    
    let mut producer = Producer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();
    
    let mut buf = String::with_capacity(2);
    for i in 0..10 {
        // info!("for: {:?}", i);

        let _ = write!(&mut buf, "{}", i); // some computation of the message data to be sent

        info!("{:?}", buf.as_bytes());

        producer.send(&Record::from_value("test.topic", buf.as_bytes())).unwrap();
        buf.clear();
    }

    info!("done");
}
