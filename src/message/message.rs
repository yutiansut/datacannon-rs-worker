/// Message struct for all connection types
/// Author: Andrew Evans

use serde_json::Value;

enum Primitives<'a>{
    String(&'a str),
    Bool,

}


///Trait to standardize messages
pub trait Message{

    ///Package the message
    fn package(&self) -> &[u8];

    ///Unpackage the message and return the headers and body json
    fn unpackage(&self) -> (Value, Value);

    ///unpackage the body of the message
    fn unpackage_result(&self) -> Value;
}

struct Header{
    name: String,
    value: Value,
}


struct RabbitMQMessage{
    headers: [Header],
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_message(){

    }

    #[test]
    fn should_unpackage_message(){

    }

    #[test]
    fn should_unpackage_result(){

    }

    #[test]
    fn should_(){

    }
}

