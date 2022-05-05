extern crate log;

use log::{ info, error };

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

fn main() {
    env_logger::init();
    info!("starting up ...");

    // let mut consumer = Consumer::from_hosts(vec!("localhost:9092".to_owned()))
    //   .with_topic_partitions("my-topic".to_owned(), &[0, 1])
    //   .with_fallback_offset(FetchOffset::Earliest)
    //   .with_group("my-group".to_owned())
    //   .with_offset_storage(GroupOffsetStorage::Kafka)
    //   .create()
    //   .unwrap();

    match Consumer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_topic_partitions("my-topic-2".to_owned(), &[0, 1])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("my-group".to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create() {

        Ok(mut consumer) => {
            loop {
                for ms in consumer.poll().unwrap().iter() {
                    for m in ms.messages() {
                        // println!("{:?}", m);
                        info!("message: {:?}", m);
                    }
                    if let Err(e) = consumer.consume_messageset(ms) {
                        error!("ERROR: {:?}", e);
                    }
                }
                consumer.commit_consumed().unwrap();
            }
        }
        Err(e) => {
            error!("ERROR: {:?}", e);
        }
    }
}
