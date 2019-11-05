/*
Get an anonymous nodename for celery
*/


use std::process;
use hostname;


/// get the naodename
pub fn get_anon_nodename(host: Option<String>, prefix: Option<String>) -> String{
    let mut hname = "".to_string();
    let mut pname = "gen".to_string();
    if host.is_some() {
        hname = host.unwrap().clone();
    }else {
        hname = hostname::get_hostname().unwrap();
    }

    if prefix.is_some(){
        pname = prefix.unwrap().clone();
    }
    let pid = process::id();
    let node_name = format!("{}{}@{}", pname, pid, hname);
    node_name
}