/*
Broker message structur3e

Author Andrew Evans
*/

use std::vec;


pub struct BrokerMessage{
    message: String,
}


impl BrokerMessage{

    fn serialize() -> vec<u8>{
        let byte_vec = vec::<u8>::empty();
        byte_vec
    }

    fn new(message: String) -> BrokerMessage{
        BrokerMessage{
            message: message,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_serialize_message(){

    }
}
