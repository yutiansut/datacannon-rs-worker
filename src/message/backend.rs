/*
Backend message struct

Author Andrew Evans
*/

pub struct BackendMessage{
}

pub struct ReturnMessage{
    message: String,
}


impl BackendMessage{

    fn new() -> BackendMessage{
        BackendMessage{}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_deserialize_message(){

    }
}