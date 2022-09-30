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

use common::mail::Mail;

use mailer::Mailer;


const QUEUE_NAME: &str = "queue";
const TOPIC: &str = "mailer";
const GROUP: &str = "mailer";


fn main() {
    env_logger::init();


    println!("started qmailer consumer");

    if let Some(cfg) = get_configuration() {
        let mut mailer = Mailer::new(
            &cfg.mailer.host,
            &cfg.mailer.user,
            &cfg.mailer.password
        );

        if let Err(e) = mailer.connect() {
            error!("unable to connect to smtp server: {:?}", e);
        }

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
                                if let Ok(sz) = str::from_utf8(m.value) {
                                    // let v: Value = serde_json::from_str(sz).unwrap();
                                    match serde_json::from_str::<Mail>(sz) {
                                        Err(e) => {
                                            error!("unable to parse mail data: {:?}", sz);
                                        }
                                        Ok(mail) => {
                                            debug!("mail data: {:?}", mail);

                                            if let Err(e) = mailer.send(
                                                &cfg.mailer.user,
                                                &mail.to,
                                                &mail.subject,
                                                &mail.body
                                            ) {
                                                error!("unable to send email: {:?}", e);
                                            } else {
                                                info!("email sent!");
                                            }
                                        }
                                    }
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
