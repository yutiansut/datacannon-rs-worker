/*
Properties for the message

Author Andrew Evans
*/

use amiquip::AmqpProperties;
use uuid::Uuid;


/// Properties
pub struct Properties{
    pub correlation_id: String,
    pub content_type: String,
    pub content_encoding: String,
    pub reply_to: Option<String>,
}


impl Properties{

    /// convert to amqp properties
    pub fn convert_to_amqp_properties(&self) -> AmqpProperties{
        let uid =  Uuid::new_v4();
        let message_id = format!("{}", uid);
        let mut props = AmqpProperties::default();
        props = props.with_message_id(message_id);
        props = props.with_correlation_id(self.correlation_id.clone());
        props = props.with_content_type(self.content_type.clone());
        props = props.with_content_encoding(self.content_encoding.clone());
        if self.reply_to.is_some() {
            let rt = self.reply_to.clone().unwrap();
            props = props.with_reply_to(rt);
        }
        props
    }

    /// Create a new properties
    pub fn new(correlation_id: String, content_type: String, content_encoding: String, reply_to: Option<String>) -> Properties{
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
    use crate::broker::properties::Properties;

    #[test]
    fn should_convert_to_amqp_properties(){
        let correlation_id = String::from("test_correlation");
        let content_type = String::from("test_content");
        let content_encoding = String::from("test_encoding");
        let props = Properties::new(correlation_id, content_type, content_encoding, None);
        let aprops = props.convert_to_amqp_properties();
        assert!(aprops.correlation_id().is_some());
        assert!(aprops.correlation_id().to_owned().unwrap().eq("test_correlation"));
        assert!(aprops.content_type().is_some());
        assert!(aprops.content_type().to_owned().unwrap().eq("test_content"));
        assert!(aprops.content_encoding().is_some());
        assert!(aprops.content_encoding().to_owned().unwrap().eq("test_encoding"));
    }

    #[test]
    fn should_create_new_properties(){
        let correlation_id = String::from("test_correlation");
        let content_type = String::from("test_content");
        let content_encoding = String::from("test_encoding");
        let props = Properties::new(correlation_id, content_type, content_encoding, None);
        assert!(props.correlation_id.eq("test_correlation"));
        assert!(props.content_type.eq("test_content"));
        assert!(props.content_encoding.eq("test_encoding"));
        assert!(props.reply_to.is_none());
    }
}
