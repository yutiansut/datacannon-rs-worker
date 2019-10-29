/*
Parser given arguments into a json string

Author Andrew Evans
*/

use amiquip::AmqpValue;
use std::vec::Vec;


/// Structure storing the arguments
pub struct Args{
    pub args: Vec<Arg>,
}


/// Struct storing a single argument
#[derive(Clone, Debug)]
pub struct Arg{
    pub arg: String,
    pub arg_type: AmqpValue,
}


/// Argument implementation
impl Arg{

    /// Create a new argument
    fn new(arg: String, arg_type: AmqpValue) -> Arg{
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


    #[test]
    fn should_create_an_argument(){
        let test_string = String::from("test");
        let arg = Arg::new(test_string.clone(), AmqpValue::LongString(test_string.clone()));
        assert!(arg.arg.eq("test_string"));
    }

    #[test]
    fn should_create_an_argument_list(){
        let test_string = String::from("test");
        let arg = Arg::new(test_string.clone(), AmqpValue::LongString(test_string.clone()));
        assert!(arg.arg.eq("test"));
        let test_int_arg = format!("{}", 0);
        let arg_b = Arg::new(test_int_arg, AmqpValue::ShortInt(0));
        assert!(arg_b.arg.eq("0"));
        let i = arg_b.arg.parse::<i8>().unwrap();
        assert!(i == 0);
        let mut args = Args::new();
        args.add_arg(arg);
        args.add_arg(arg_b);
        assert!(args.size() == 2);
        assert!(args.args.len() == 2);
        assert!(args.args[0].arg.eq("test"));
    }
}
