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
}


/// functions for converting message to string
impl Message{

    /// convert the body to json
    fn new(properties: Properties, headers: Headers, body: MessageBody) -> Message{
        unimplemented!()
    }
}


#[cfg(test)]
mod tests{


    #[test]
    fn should_get_kwargs_repr(){

    }

    #[test]
    fn should_get_args_repr(){

    }

    #[test]
    fn should_convert_body_to_json(){

    }

    #[test]
    fn should_convert_message_to_amiquip_objects(){

    }
}
