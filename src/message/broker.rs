/*
Broker message structur3e

Author Andrew Evans
*/

use amiquip::{AmqpValue, Publish, AmqpProperties};
use std::vec;
use crate::config::config::CeleryConfig;


/// The message sent to the broker
pub struct BrokerMessage<'a>{
    message_bytes: &'a [u8],
    properties: Option<AmqpProperties>,
}


/// Broker implementation
impl <'a> BrokerMessage<'a>{

    /// create a new message
    fn new(message_bytes: &[u8], properties: Option<AmqpProperties>) -> BrokerMessage{
        BrokerMessage{
            message_bytes: message_bytes,
            properties: properties,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn should_create_and_access_message(){
        let mut om = String::from("No");
        {
            let m = "Hello".as_bytes();
            let bm = BrokerMessage::new(m, None);
            om = String::from_utf8_lossy(bm.message_bytes).to_string();
        }
        assert!(om.eq("Hello"));
    }

    #[test]
    fn should_create_threadable_message(){
        let om = String::from("No");
        let m = "Hello".as_bytes();
        let _t = thread::spawn(move || -> Result<String, ()>{
            let bm = BrokerMessage::new(m, None);
            let om = String::from_utf8_lossy(bm.message_bytes).to_string();
            assert!(om.eq("Hello"));
            Ok(om)
        });
        let res = _t.join();
        let rstr = res.ok().unwrap().ok().unwrap();
        assert!(rstr.eq("Hello"));
    }
}
