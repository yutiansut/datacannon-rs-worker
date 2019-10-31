/*
Headers for the message

Author Andrew Evans
*/

use amiquip::AmqpValue;
use crate::argparse::{args::Args, kwargs::KwArgs};
use serde_json::{Value, Map, Number};
use std::collections::BTreeMap;
use amq_protocol::uri::AMQPScheme::AMQP;


/// Soft and hard time limits
#[derive(Clone, Debug)]
pub struct TimeLimit{
    pub soft: i64,
    pub hard: i64,
}


/// Stored headers
#[derive(Clone, Debug)]
pub struct Headers{
    pub lang: String,
    pub task: String,
    pub id: String,
    pub root_id: String,
    pub parent_id: String,
    pub group: String,
    pub meth: Option<String>,
    pub shadow: Option<String>,
    pub eta: Option<String>,
    pub expires: Option<String>,
    pub retries: Option<i8>,
    pub timelimit: Option<TimeLimit>,
    pub argsrepr: Option<Args>,
    pub kwargsrepr: Option<KwArgs>,
    pub origin: Option<String>,
}


/// Headers implementation
impl Headers{

    /// convert headers to a btree map
    pub fn convert_to_btree_map(&self) -> BTreeMap<String, AmqpValue>{
        let mut jmap = BTreeMap::<String, AmqpValue>::new();
        jmap.insert(String::from("lang"), AmqpValue::LongString(self.lang.clone()));
        jmap.insert(String::from("task"), AmqpValue::LongString(self.task.clone()));
        jmap.insert(String::from("id"), AmqpValue::LongString(self.id.clone()));
        jmap.insert(String::from("root_id"), AmqpValue::LongString(self.root_id.clone()));
        jmap.insert(String::from("parent_id"), AmqpValue::LongString(self.parent_id.clone()));
        jmap.insert(String::from("group"), AmqpValue::LongString(self.group.clone()));

        if self.meth.is_some() {
            let v = self.meth.clone().unwrap();
            jmap.insert(String::from("meth"), AmqpValue::LongString(v));
        }

        if self.shadow.is_some(){
            let v = self.shadow.clone().unwrap();
            jmap.insert(String::from("shadow"), AmqpValue::LongString(v));
        }

        if self.eta.is_some(){
            let v = self.eta.clone().unwrap();
            jmap.insert(String::from("eta"), AmqpValue::LongString(v));
        }

        if self.expires.is_some(){
            let v = self.expires.clone().unwrap();
            jmap.insert(String::from("expires"), AmqpValue::LongString(v));
        }

        if self.retries.is_some(){
            let v = self.retries.clone().unwrap();
            jmap.insert(String::from("retries"), AmqpValue::ShortShortInt(v));
        }

        if self.timelimit.is_some(){
            let v = self.timelimit.clone().unwrap();
            let vtup = vec![AmqpValue::LongLongInt(v.soft), AmqpValue::LongLongInt(v.hard)];
            jmap.insert(String::from("timelimit"), AmqpValue::FieldArray(vtup));
        }

        if self.argsrepr.is_some(){
            let v = self.argsrepr.clone().unwrap();
            let argsrepr = v.args_to_vec();
            jmap.insert(String::from("args"), AmqpValue::FieldArray(v.args_to_amqp_vec()));
        }

        if self.kwargsrepr.is_some(){
            let v = self.kwargsrepr.clone().unwrap();
            let vm = v.convert_to_btree_map();
            jmap.insert(String::from("kwargsrepr"), AmqpValue::FieldTable(vm));
        }


        if self.origin.is_some(){
            let v = self.origin.clone().unwrap();
            jmap.insert(String::from("origin"), AmqpValue::LongString(v));
        }
        jmap
    }

    /// convert to a json capable map
    pub fn convert_to_json_map(&self) -> Map<String, Value>{
        let mut jmap = Map::new();
        jmap.insert(String::from("lang"), Value::String(self.lang.clone()));
        jmap.insert(String::from("task"), Value::String(self.task.clone()));
        jmap.insert(String::from("id"), Value::String(self.id.clone()));
        jmap.insert(String::from("root_id"), Value::String(self.root_id.clone()));
        jmap.insert(String::from("parent_id"), Value::String(self.parent_id.clone()));
        jmap.insert(String::from("group"), Value::String(self.group.clone()));

        if self.meth.is_some() {
            let v = self.meth.clone().unwrap();
            jmap.insert(String::from("meth"), Value::from(v));
        }

        if self.shadow.is_some(){
            let v = self.shadow.clone().unwrap();
            jmap.insert(String::from("shadow"), Value::from(v));
        }

        if self.eta.is_some(){
            let v = self.eta.clone().unwrap();
            jmap.insert(String::from("eta"), Value::from(v));
        }

        if self.expires.is_some(){
            let v = self.expires.clone().unwrap();
            jmap.insert(String::from("expires"), Value::from(v));
        }

        if self.retries.is_some(){
            let v = self.retries.clone().unwrap();
            jmap.insert(String::from("retries"), Value::from(v));
        }

        if self.timelimit.is_some(){
            let v = self.timelimit.clone().unwrap();
            let vtup = vec![Value::from(v.soft), Value::from(v.hard)];
            jmap.insert(String::from("timelimit"), Value::Array(vtup));
        }

        if self.argsrepr.is_some(){
            let v = self.argsrepr.clone().unwrap();
            let argsrepr = v.args_to_vec();
            jmap.insert(String::from("args"), Value::Array(argsrepr));
        }

        if self.kwargsrepr.is_some(){
            let v = self.kwargsrepr.clone().unwrap();
            let vm = v.convert_to_map();
            jmap.insert(String::from("kwargsrepr"), Value::Object(vm));
        }


        if self.origin.is_some(){
            let v = self.origin.clone().unwrap();
            jmap.insert(String::from("origin"), Value::from(v));
        }
        jmap
    }

    /// create new headers
    pub fn new(lang: String, task: String, id: String, root_id: String, parent_id: String, group: String) -> Headers{
        Headers{
            lang: lang,
            task: task,
            id: id,
            root_id: root_id,
            parent_id: parent_id,
            group: group,
            meth: None,
            shadow: None,
            eta: None,
            expires: None,
            retries: None,
            timelimit: None,
            argsrepr: None,
            kwargsrepr: None,
            origin: None,
        }
    }
}


#[cfg(test)]
mod tests{
    use crate::broker::headers::Headers;
    use crate::argparse::args::{Arg, Args};
    use std::vec::Vec;

    #[test]
    fn should_convert_to_json_map(){
        let mut h = Headers::new(String::from("rs"), String::from("test_task"), String::from("id"), String::from("test_root"), String::from("parent_id"), String::from("group"));
        let arep = Args{
            args: Vec::<Arg>::new(),
        };
        h.argsrepr = Some(arep);
        let m = h.convert_to_json_map();
        let l = m.get("lang");
        assert!(String::from(l.unwrap().as_str().unwrap()).eq("rs"));
    }

    #[test]
    fn should_create_new_headers(){
        let mut h = Headers::new(String::from("rs"), String::from("test_task"), String::from("id"), String::from("test_root"), String::from("parent_id"), String::from("group"));
        let arep = Args{
          args: Vec::<Arg>::new(),
        };
        h.argsrepr = Some(arep);
        let lang = h.lang;
        let optrep = h.argsrepr;
        assert!(lang.eq("rs"));
        assert!(optrep.unwrap().args.len() == 0);
    }
}
