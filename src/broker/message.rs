/*
A message body for

Author Andrew Evans
*/

use amiquip::{AmqpProperties, AmqpValue};

use crate::argparse::kwargs::KwArgs;
use crate::argparse::args::Args;
use crate::broker::{properties::Properties, message_body::MessageBody, headers::Headers};
use serde_json::{Value, Map, to_string};
use amq_protocol::uri::AMQPScheme::AMQP;


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

        /// get extra properties
        let jheaders = self.headers.convert_to_btree_map();
        props = props.with_headers(jheaders);

        /// get the message body string
        let mut body_vec = Vec::<Value>::new();

        if self.args.is_some() {
            let args = self.args.clone().unwrap().args_to_vec();
            body_vec.push(Value::Array(args));
        }else{
            body_vec.push(Value::Null);
        }

        if self.kwargs.is_some(){
            let kwargs = self.kwargs.clone().unwrap().convert_to_map();
            body_vec.push(Value::Object(kwargs));
        }else{
            body_vec.push(Value::Null);
        }

        let message_map = self.body.convert_to_json_map();
        let mbody_val = Value::Object(message_map);
        body_vec.push(mbody_val);
        let bv = Value::Array(body_vec);
        let body_str = to_string(&bv).ok().unwrap();
        (body_str, props)
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
    use super::*;
    use serde_json::{Value, from_str};
    use crate::broker::properties::Properties;
    use crate::broker::headers::Headers;
    use crate::argparse::args::{Args, Arg};
    use crate::broker::message_body::MessageBody;
    use crate::argparse::kwargs::KwArg;
    use amq_protocol::types::AMQPType;
    use amiquip::AmqpValue;

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
        let mb = MessageBody::new(Some(String::from("chord")), None, None, None);
        let cjm = mb.convert_to_json_map();
        let ch = cjm.get("chord");
        let cv = ch.unwrap().to_owned();
        let test_string = String::from("test");
        let test_val = Value::from(test_string);
        let arg = Arg::new(test_val.clone(), AMQPType::LongString);
        assert!(arg.arg.as_str().unwrap().eq("test"));
        let mut args = Args::new();
        args.args.push(arg);
        let kwargs: Option<KwArg> = None;
        let m = Message::new(props, h, mb, Some(args), None);
    }

    #[test]
    fn test_serialize_body(){
        let correlation_id = String::from("test_correlation");
        let content_type = String::from("test_content");
        let content_encoding = String::from("test_encoding");
        let props = Properties::new(correlation_id, content_type, content_encoding, None);
        let mut h = Headers::new(String::from("rs"), String::from("test_task"), String::from("id"), String::from("test_root"), String::from("parent_id"), String::from("group"));
        let arep = Args{
            args: Vec::<Arg>::new(),
        };
        h.argsrepr = Some(arep);
        let mb = MessageBody::new(Some(String::from("chord")), None, None, None);
        let cjm = mb.convert_to_json_map();
        let ch = cjm.get("chord");
        let cv = ch.unwrap().to_owned();
        let test_string = String::from("test");
        let test_val = Value::from(test_string);
        let arg = Arg::new(test_val.clone(), AMQPType::LongString);
        assert!(arg.arg.as_str().unwrap().eq("test"));
        let mut args = Args::new();
        args.args.push(arg);
        let kwargs: Option<KwArg> = None;
        let m = Message::new(props, h, mb, Some(args), None);
        let (body, props) = m.get_message_parts();
        let jval = from_str(body.as_str());
        let rval: Value = jval.ok().unwrap();
        let o = rval.as_array().unwrap().to_owned();
        assert!(o.len() == 3);
        let a1 = o.get(0).unwrap().to_owned();
        let jargs = a1.as_array().unwrap().to_owned();
        assert!(jargs.len() == 1);
        assert!(jargs.get(0).unwrap().eq("test"));
        assert!(o.get(1).unwrap().to_owned() == Value::Null);
    }
}
