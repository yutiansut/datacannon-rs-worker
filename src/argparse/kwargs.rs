/*
Kwargs for the program

Author Andrew Evans
*/

use amiquip::AmqpValue;
use amq_protocol::types::AMQPType;
use std::vec::Vec;

use crate::argparse::args::Arg;
use std::iter::Map;
use std::collections::HashMap;
use serde_json::Value;


/// Keyword argument
#[derive(Clone, Debug)]
pub struct KwArg{
    key: String,
    arg: Arg,
}


/// Structure storing the arguments
pub struct KwArgs{
    pub kwargs: Vec<KwArg>,
}


/// Argument implementation
impl KwArg{

    /// Create a new argument
    fn new(key: String, arg: Value, arg_type: AMQPType) -> KwArg{
        let arg_object = Arg{
            arg: arg,
            arg_type: arg_type,
        };

        KwArg{
            key: key,
            arg: arg_object,
        }
    }
}


/// Implementation of arguments list
impl KwArgs{

    /// size of the list
    fn size(&self) -> usize{
        self.kwargs.len()
    }

    /// add an argument
    fn add_kwarg(&mut self, kwarg: KwArg){
        self.kwargs.push(kwarg);
    }

    /// create a new arguments list
    fn new() -> KwArgs{
        KwArgs{
            kwargs: Vec::<KwArg>::new(),
        }
    }

}


#[cfg(test)]
mod tests{

    use super::*;
    use amq_protocol::types::AMQPType;
    use serde_json::Value;


    #[test]
    fn should_create_an_kwarg_argument(){
        let kstr = String::from("test_key");
        let kvalstr = String::from("test_val");
        let kval = Value::from(kvalstr);
        let arg = Arg{
            arg: kval.clone().to_owned(),
            arg_type: AMQPType::LongString,
        };

        let kwarg = KwArg{
            key: kstr.to_owned(),
            arg: arg.to_owned(),
        };
        let k = kwarg.key;
        let v = kwarg.arg.arg;
        assert!(k.eq("test_key"));
        assert!(v.as_str().unwrap().eq("test_val"));
    }

    #[test]
    fn should_create_a_kwargs_list(){
        let kstr = String::from("test_key");
        let kvalstr = String::from("test_val");
        let kval = Value::from(kvalstr);
        let arg = Arg{
            arg: kval.clone().to_owned(),
            arg_type: AMQPType::LongString,
        };

        let kwarg = KwArg{
            key: kstr.to_owned(),
            arg: arg.to_owned(),
        };
        let kstrb = String::from("test_keyb");
        let kvstrb = String::from("test_val");
        let vstrb = Value::from(kvstrb);
        let argb = Arg{
            arg: vstrb.clone().to_owned(),
            arg_type: AMQPType::LongString,
        };
        let kwargb = KwArg{
            key: kstrb.to_owned(),
            arg: argb.clone(),
        };
        let mut kwargs = KwArgs::new();
        kwargs.add_kwarg(kwarg.clone());
        kwargs.add_kwarg(kwargb.clone());
        assert!(kwargs.size() == 2);
        assert!(kwargs.kwargs.len() == 2);
    }
}
