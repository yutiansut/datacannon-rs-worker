/*
Properties for the message

Author Andrew Evans
*/

use amiquip::AmqpProperties;


/// Properties
pub struct Properties{
    pub correlation_id: String,
    pub content_type: String,
    pub content_encoding: String,
    pub reply_to: Option<String>,
}


impl Properties{

    fn convert_to_amqp_properties(&self) -> AmqpProperties{
        unimplemented!()
    }

    /// Create a new properties
    fn new(correlation_id: String, content_type: String, content_encoding: String, reply_to: Option<String>) -> Properties{
        Properties{
            correlation_id: correlation_id,
            content_type: content_type,
            content_encoding: content_encoding,
            reply_to: reply_to,
        }
    }
}


#[cfg(test)]
mod tests{

    #[test]
    fn should_convert_to_amqp_properties(){

    }

    #[test]
    fn should_package_properties(){

    }
}
