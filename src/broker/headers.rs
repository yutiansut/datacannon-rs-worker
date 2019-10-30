/*
Headers for the message

Author Andrew Evans
*/

use crate::argparse::{args::Args, kwargs::KwArgs};
use serde_json::{Value, Map};


/// Soft and hard time limits
pub struct TimeLimit{
    pub soft: i64,
    pub hard: i64,
}


/// Stored headers
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

    /// convert to a json capable map
    fn convert_to_json_map(&self) -> Map<String, Value>{
        unimplemented!()
    }

    /// create new headers
    fn new(lang: String, task: String, id: String, root_id: String, parent_id: String, group: String) -> Headers{
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

    #[test]
    fn should_convert_to_json_map(){

    }

    #[test]
    fn should_create_new_headers(){

    }
}
