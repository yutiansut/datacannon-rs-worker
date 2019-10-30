/*
Parser given arguments into a json string

Author Andrew Evans
*/

use amiquip::AmqpValue;
use amq_protocol::types::AMQPType;
use std::vec::Vec;
use serde_json::Value;


/// Structure storing the arguments
pub struct Args{
    pub args: Vec<Arg>,
}


/// Struct storing a single argument
#[derive(Clone, Debug)]
pub struct Arg{
    pub arg: Value,
    pub arg_type: AMQPType,
}


/// Argument implementation
impl Arg{

    /// Create a new argument
    fn new(arg: Value, arg_type: AMQPType) -> Arg{
        Arg{
            arg: arg,
            arg_type: arg_type
        }
    }
}


/// Implementation of arguments list
impl Args{

    /// size of the list
    fn size(&self) -> usize{
        self.args.len()
    }

    /// add an argument
    fn add_arg(&mut self, arg: Arg){
        self.args.push(arg);
    }

    /// create a new arguments list
    fn new() -> Args{
        Args{
           args: Vec::<Arg>::new(),
        }
    }

}


#[cfg(test)]
mod tests{

    use super::*;

    use serde_json::Value;
    use amq_protocol::types::AMQPType;


    #[test]
    fn should_create_an_argument(){
        let test_string = String::from("test");
        let test_val = Value::from(test_string);
        let arg = Arg::new(test_val.clone(), AMQPType::LongString);
        assert!(arg.arg.as_str().unwrap().eq("test"));
    }

    #[test]
    fn should_create_an_argument_list(){
        let test_string = String::from("test");
        let test_val = Value::from(test_string);
        let arg = Arg::new(test_val.clone(), AMQPType::LongString);
        assert!(arg.arg.as_str().unwrap().eq("test"));
        let mut args = Args::new();
        args.args.push(arg);
        assert!(args.size() == 1);
        assert!(args.args.len() == 1);
        assert!(args.args.get(0).unwrap().arg.as_str().unwrap().eq("test"));
    }
}
