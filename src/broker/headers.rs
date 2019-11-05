/*
Headers for the message

Author Andrew Evans
*/

use std::collections::BTreeMap;

use amiquip::AmqpValue;
use amq_protocol::uri::AMQPScheme::AMQP;
use serde_json::{Map, Number, to_string, Value};
use uuid::Uuid;

use crate::argparse::{args::Args, kwargs::KwArgs};
use crate::nodename::anon_name;
use std::process;

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
    pub parent_id: Option<String>,
    pub group: Option<String>,
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

        if self.parent_id.is_some() {
            jmap.insert(String::from("parent_id"), AmqpValue::LongString(self.parent_id.clone().unwrap()));
        }else {
            jmap.insert("parent_id".to_string(), AmqpValue::Void);
        }

        if self.group.is_some() {
            jmap.insert(String::from("group"), AmqpValue::LongString(self.group.clone().unwrap()));
        }else{
            jmap.insert("group".to_string(), AmqpValue::Void);
        }

        if self.meth.is_some() {
            let v = self.meth.clone().unwrap();
            jmap.insert(String::from("meth"), AmqpValue::LongString(v));
        }

        if self.shadow.is_some(){
            let v = self.shadow.clone().unwrap();
            jmap.insert(String::from("shadow"), AmqpValue::LongString(v));
        }else{
            jmap.insert("shadow".to_string(), AmqpValue::Void);
        }

        if self.eta.is_some(){
            let v = self.eta.clone().unwrap();
            jmap.insert(String::from("eta"), AmqpValue::LongString(v));
        }else{
            jmap.insert("eta".to_string(), AmqpValue::Void);
        }

        if self.expires.is_some(){
            let v = self.expires.clone().unwrap();
            jmap.insert(String::from("expires"), AmqpValue::LongString(v));
        }else{
            jmap.insert("expires".to_string(), AmqpValue::Void);
        }

        if self.retries.is_some(){
            let v = self.retries.clone().unwrap();
            jmap.insert(String::from("retries"), AmqpValue::ShortShortInt(v));
        }else{
            jmap.insert("retries".to_string(), AmqpValue::Void);
        }

        if self.timelimit.is_some(){
            let v = self.timelimit.clone().unwrap();
            let vtup = vec![AmqpValue::LongLongInt(v.soft), AmqpValue::LongLongInt(v.hard)];
            jmap.insert(String::from("timelimit"), AmqpValue::FieldArray(vtup));
        }else{
            let vtup = vec![AmqpValue::Void, AmqpValue::Void];
            jmap.insert("timelimit".to_string(), AmqpValue::FieldArray(vtup));
        }

        if self.argsrepr.is_some(){
            let v = self.argsrepr.clone().unwrap();
            let val = v.args_to_vec();
            let jstr = to_string(&Value::Array(val));
            jmap.insert("argsrepr".to_string(), AmqpValue::LongString(jstr.unwrap()));
        }else{
            let v = Value::from(Vec::<Value>::new());
            let vm = to_string(&v);
            jmap.insert("argsrepr".to_string(), AmqpValue::LongString(vm.unwrap()));
        }

        if self.kwargsrepr.is_some(){
            let v = self.kwargsrepr.clone().unwrap();
            let vm = to_string(&Value::from(v.convert_to_map()));
            if vm.is_ok() {
                let jstr = vm.unwrap();
                jmap.insert(String::from("kwargsrepr"), AmqpValue::LongString(jstr));
            }
        }else{
            let v = Map::<String, Value>::new();
            let vm = to_string(&Value::from(v));
            if vm.is_ok() {
                let jstr = vm.unwrap();
                jmap.insert("kwargsrepr".to_string(), AmqpValue::LongString(jstr));
            }
        }


        if self.origin.is_some(){
            let v = self.origin.clone().unwrap();
            jmap.insert(String::from("origin"), AmqpValue::LongString(v));
        }else{
            let nodename = anon_name::get_anon_nodename(None, None);
            jmap.insert("origin".to_string(), AmqpValue::LongString(nodename));
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

        if self.parent_id.is_some() {
            let parent_id = self.parent_id.clone().unwrap();
            jmap.insert(String::from("parent_id"), Value::String(parent_id));
        }

        if self.group.is_some() {
            let group = self.group.clone().unwrap();
            jmap.insert(String::from("group"), Value::String(group));
        }

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
    pub fn new(lang: String, task: String, id: String, root_id: String) -> Headers{
        Headers{
            lang: lang,
            task: task,
            id: id,
            root_id: root_id,
            parent_id: None,
            group: None,
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
    use std::vec::Vec;

    use crate::argparse::args::{Arg, Args};
    use crate::broker::headers::Headers;

    #[test]
    fn should_convert_to_json_map(){
        let mut h = Headers::new(String::from("rs"), String::from("test_task"), String::from("id"), String::from("test_root"));
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
        let mut h = Headers::new(String::from("rs"), String::from("test_task"), String::from("id"), String::from("test_root"));
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
