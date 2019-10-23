/*
Backend message struct

Author Andrew Evans
*/


pub struct BackendMessage{
    message: vec<u8>,
}

pub struct ReturnMessage{
    message: String,
}


impl BackendMessage{

    fn deserialize_message(&self) -> ReturnMessage{
        ReturnMessage{
            message: String::from("unimplemented"),
        }
    }

    fn new(message: vec<u8>) -> BackendMessage{
        BackendMessage{
            message: message,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_deserialize_message(){

    }
}