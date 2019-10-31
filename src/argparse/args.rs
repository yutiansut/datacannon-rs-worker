/*
Parser given arguments into a json string

Author Andrew Evans
*/

use amiquip::AmqpValue;
use amq_protocol::types::AMQPType;
use std::vec::Vec;
use serde_json::Value;
use amq_protocol::uri::AMQPScheme::AMQP;


/// Structure storing the arguments
#[derive(Clone, Debug)]
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
    pub fn new(arg: Value, arg_type: AMQPType) -> Arg{
        Arg{
            arg: arg,
            arg_type: arg_type
        }
    }
}


/// Implementation of arguments list
impl Args{

    /// convert to amqp vec
    pub fn args_to_amqp_vec(&self) -> Vec<AmqpValue>{
        let mut val_vec = Vec::<AmqpValue>::new();
        for i in 0..self.args.len(){
            let val = self.args.get(i).unwrap().clone();
            //val_vec.push(Value::from(val));
        }
        val_vec
    }

    /// convert args to a vec
    pub fn args_to_vec(&self) -> Vec<Value>{
        let mut val_vec = Vec::<Value>::new();
        for i in 0..self.args.len(){
            let val = self.args.get(i).unwrap().clone();
            val_vec.push(val.arg);
        }
        val_vec
    }

    /// size of the list
    pub fn size(&self) -> usize{
        self.args.len()
    }

    /// add an argument
    pub fn add_arg(&mut self, arg: Arg){
        self.args.push(arg);
    }

    /// create a new arguments list
    pub fn new() -> Args{
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
