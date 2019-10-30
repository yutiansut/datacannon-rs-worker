/*
Message body for the broker

Author Andrew Evans
*/

use serde_json::{Value, Map};


/// Message body structure
pub struct MessageBody{
    pub chord: Option<String>,
    pub chain: Option<String>,
    pub callbacks: Option<Vec<String>>,
    pub errbacks: Option<Vec<String>>,
}


/// implementation of message body
impl MessageBody{

    /// covnert to a json map
    fn convert_to_json_map(&self) -> Map<String, Value>{
        unimplemented!()
    }

    /// create a new message body
    fn new(chord: Option<String>, chain: Option<String>, callbacks: Option<Vec<String>>, errbacks: Option<Vec<String>>) -> MessageBody{
        MessageBody{
            chord: chord,
            chain: chain,
            callbacks: callbacks,
            errbacks: errbacks,
        }
    }
}


#[cfg(test)]
mod tests{

    #[test]
    fn should_convert_to_json_map(){

    }

    #[test]
    fn should_create_new_message_body(){

    }
}

