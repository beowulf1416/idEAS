use log::{ info, error };
// use std::fmt::Write;
use std::time::Duration;

use kafka::producer::{ Consumer, Record, RequiredAcks };

use serde_json::{ Value };


pub struct Subscriber {
    subscriber: subscriber
}

impl Subscriber {

    pub fn new(hosts: Vec<String>) -> Self {

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