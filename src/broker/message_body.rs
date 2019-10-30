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
    pub fn convert_to_json_map(&self) -> Map<String, Value>{
        let mut m = Map::new();
        if self.chord.is_some() {
            m.insert(String::from("chord"), Value::from(self.chord.clone().unwrap()));
        }else{
            m.insert(String::from("chord"), Value::Null);
        }
        if self.chain.is_some() {
            m.insert(String::from("chain"), Value::from(self.chain.clone().unwrap()));
        }else{
            m.insert(String::from("chain"), Value::Null);
        }
        if self.callbacks.is_some() {
            m.insert(String::from("callbacks"), Value::from(self.callbacks.clone().unwrap()));
        }else{
            m.insert(String::from("callbacks"), Value::Null);
        }
        if self.errbacks.is_some() {
            m.insert(String::from("errbacks"), Value::from(self.errbacks.clone().unwrap()));
        }else{
            m.insert(String::from("errbacks"), Value::Null);
        }
        m
    }

    /// create a new message body
    pub fn new(chord: Option<String>, chain: Option<String>, callbacks: Option<Vec<String>>, errbacks: Option<Vec<String>>) -> MessageBody{
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
    use crate::broker::message_body::MessageBody;

    #[test]
    fn should_convert_to_json_map(){
        let mb = MessageBody::new(Some(String::from("chord")), None, None, None);
        let cjm = mb.convert_to_json_map();
        let ch = cjm.get("chord");
        let cv = ch.unwrap().to_owned();
        assert!(String::from(cv.as_str().unwrap()).eq("chord"));
    }

    #[test]
    fn should_create_new_message_body(){
        let mb = MessageBody::new(None, None, None, None);
        assert!(mb.errbacks.is_none());
    }
}

