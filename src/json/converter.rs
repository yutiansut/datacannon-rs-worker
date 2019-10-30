/*
Json converter for sending a message

Author Andrew Evans
*/

use amiquip::AmqpValue;
use amq_protocol::types::AMQPType;
use serde_json::Value;


/// Add a value to a list


/// Add to mapping



#[cfg(test)]
mod tests{
    use super::*;
    use serde_json;
    use serde_json::Map;

    #[test]
    fn test_serde_capabilities(){
        let mut m = serde_json::Map::new();
        let kstr = String::from("test");
        let v = Value::from("val");
        m.insert(kstr, v);
        let mstr =  serde_json::to_string(&m).unwrap();
        let mstruct: Map<String, Value> = serde_json::from_str(&mstr).unwrap();
        let mopt = mstruct.get("test");
        assert!(mopt.is_some());
        assert!(mopt.unwrap().as_str().unwrap().eq("val"));
    }
}
