/*
A message body for

Author Andrew Evans
*/

use amiquip::AmqpProperties;

use crate::argparse::kwargs::KwArgs;
use crate::argparse::args::Args;
use crate::broker::{properties::Properties, message_body::MessageBody, headers::Headers};
use serde_json::{Value, Map};


/// Message objects to be packaged when ready
pub struct Message{
    pub properties: Properties,
    pub headers: Headers,
    pub body: MessageBody,
    pub args: Option<Args>,
    pub kwargs: Option<KwArgs>,
}


/// functions for converting message to string
impl Message{

    /// Get message parts
    pub fn get_message_parts(&self) -> (String, AmqpProperties){
        let mut props = self.properties.convert_to_amqp_properties();
        let jheaders = self.headers.convert_to_btree_map();
        props = props.with_headers(jheaders);
        (String::from(""), props)
    }

    /// convert the body to json
    pub fn new(properties: Properties, headers: Headers, body: MessageBody, args: Option<Args>, kwargs: Option<KwArgs>) -> Message{
        Message{
            properties: properties,
            headers: headers,
            body: body,
            args: args,
            kwargs: kwargs,
        }
    }
}


#[cfg(test)]
mod tests{
    use crate::broker::properties::Properties;
    use crate::broker::headers::Headers;
    use crate::argparse::args::{Args, Arg};

    #[test]
    fn create_new_message(){
        let correlation_id = String::from("test_correlation");
        let content_type = String::from("test_content");
        let content_encoding = String::from("test_encoding");
        let props = Properties::new(correlation_id, content_type, content_encoding, None);
        let mut h = Headers::new(String::from("rs"), String::from("test_task"), String::from("id"), String::from("test_root"), String::from("parent_id"), String::from("group"));
        let arep = Args{
            args: Vec::<Arg>::new(),
        };
        h.argsrepr = Some(arep);
    }
}
