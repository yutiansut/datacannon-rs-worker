/*
Utilities for handling values
*/

use amiquip::AmqpValue;
use serde_json::Value;
use std::collections::BTreeMap;


pub fn value_to_amqp_value(val: &Value) -> AmqpValue{
    if val.is_array(){
        let arr = val.clone().as_array().unwrap().to_owned();
        let mut amqp_vec = Vec::<AmqpValue>::new();
        for i in 0..arr.len(){
            let arr_val = arr.get(i).unwrap();
            let v = value_to_amqp_value(arr_val);
            amqp_vec.push(v);
        }
        AmqpValue::FieldArray(amqp_vec)
    }else if val.is_string(){
        let str = val.as_str().unwrap();
        AmqpValue::LongString(String::from(str))
    }else if val.is_boolean(){
        let b = val.clone().as_bool().unwrap();
        AmqpValue::Boolean(b)
    }else if val.is_f64(){
        let f = val.clone().as_f64().unwrap();
        AmqpValue::Double(f)
    }else if val.is_i64(){
        let i = val.clone().as_i64().unwrap();
        AmqpValue::LongLongInt(i)
    }else if val.is_u64(){
        let u = val.clone().as_u64().unwrap();
        AmqpValue::Timestamp(u)
    }else if val.is_object(){
        let mut amqp_map = BTreeMap::<String, AmqpValue>::new();
        let m = val.as_object().unwrap();
        m.keys();
        let mut it = m.to_owned().into_iter();
        let tup_opt: Option<(String, Value)> = it.next();
        let (k, v) = tup_opt.unwrap();
        let av = value_to_amqp_value(&v);
        amqp_map.insert(k, av);
        AmqpValue::FieldTable(amqp_map)
    }else if val.is_null(){
        AmqpValue::Void
    }else {
        AmqpValue::Void
    }
}

#[cfg(test)]
mod tests{
    use serde_json::Value;
    use crate::serde_utils::val_handler::value_to_amqp_value;
    use amiquip::AmqpValue;
    use amq_protocol::types::FieldArray;
    use serde_json::Map;

    #[test]
    fn should_convert_array_to_amqp_value(){
        let mut val_vec = Vec::<Value>::new();
        let val = Value::from(132);
        val_vec.push(val);
        let valb = Value::from(String::from("hello world!"));
        val_vec.push(valb);
        let varr = Value::from(val_vec);
        let conv_arr = value_to_amqp_value(&varr);
        if let AmqpValue::FieldArray(convarr) = conv_arr{
            let v = convarr.get(0).unwrap().to_owned();
            if let AmqpValue::LongLongInt(v) = v{
                assert!(v == 132);
            }else{
                assert!(false);
            }
            assert!(true);
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_convert_string_to_amqp_value(){
        let str = String::from("hello world!");
        let val = Value::from(str);
        let av = value_to_amqp_value(&val);
        if let AmqpValue::LongString(av) = av{
            let s = av.to_string();
            assert!(s.eq("hello world!"));
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_convert_boolean_to_amqp_value(){
        let b= true;
        let v = Value::from(b);
        let av = value_to_amqp_value(&v);
        if let AmqpValue::Boolean(av) = av {
            assert!(b);
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_convert_i64_to_amqp_value(){
        let i: i64 =  333;
        let v = Value::from(i);
        let av = value_to_amqp_value(&v);
        if let AmqpValue::LongLongInt(av) = av {
            assert!(av == 333);
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_convert_f64_to_amqp_value(){
        let f: f64 = 10.0;
        let v = Value::from(f);
        let av = value_to_amqp_value(&v);
        if let AmqpValue::Double(av) = av{
            assert!(f == 10.0);
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_convert_object_to_amqp_value(){
        let mut m = Map::new();
        let k = String::from("key");
        let v = Value::String(String::from("val"));
        m.insert(k, v);
        let val = Value::Object(m);
        let av = value_to_amqp_value(&val);
        if let AmqpValue::FieldTable(av) = av{
            let vopt = av.get("key");
            let rv = vopt.unwrap().to_owned();
            if let AmqpValue::LongString(rv) = rv{
               assert!(rv.eq("val"));
            }else{
                assert!(false);
            }
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_convert_null_to_amqp_value(){
        let v = Value::Null;
        let av = value_to_amqp_value( &v);
        if let AmqpValue::Void = av{
            assert!(true);
        }else{
            assert!(false);
        }
    }
}