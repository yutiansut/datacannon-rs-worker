/// A rabbit mq connection factory for obtaining rabbit mq based connections.
/// Author: Andrew Evans
use std::fmt;
use std::sync::{Arc, Mutex};
use crate::connection::rabbitmq_connection_utils::RabbitMQConnection;
use amiquip::{Channel, Connection};


///Credentials object
struct Credential{
    username: String,
    password: String,
}


///Overarching Rabbit MQ Connection Factory
pub struct RabbitMQConnectionFactory{
    protocol: String,
    host: String,
    port: i64,
    vhost: Option<String>,
}


///Implementation of the Rabbit MQ Connection Factory
impl RabbitMQConnectionFactory {

    /// Open a connection and channel. Return a connection object with blocking access
    fn create_connection_object(&self, url: String) -> Result<Option<Arc<Mutex<RabbitMQConnection>>>, &'static str> {
        let conn_result = Connection::insecure_open(url.as_str());
        if(conn_result.is_ok()){
            let mut conn = conn_result.unwrap();
            let channel_result = conn.open_channel(None);
            if(channel_result.is_ok()) {
                let channel = channel_result.unwrap();
                let conn_object = RabbitMQConnection {
                    connection: conn,
                    channel: channel,
                };
                Ok(Some(Arc::new(Mutex::new(conn_object))))
            }else{
                match channel_result{
                    Err(e) =>{
                        println!("{}", e);
                    }
                    _ => {}
                }
                Err("Failed to Establish a Channel")
            }
        }else {
            match conn_result{
                Err(e) =>{
                    println!("{}", e);
                }
                _ => {}
            }
            Err("Failed to Establish a Connection")
        }
    }

    /// Append a host if it exists
    fn append_host(&self, in_url: String) -> String{
        let mut url = in_url.clone();
        if(self.vhost.is_some()){
            let host: String = self.vhost.to_owned().unwrap();
            url = format!("{}/{}", url, host);
        }
        url
    }

    /// Create a RabbitMQ Connection
    fn create_connection(&self, credentials: Option<Credential>, is_ssl: bool) -> Result<Option<Arc<Mutex<RabbitMQConnection>>>, &'static str> {
        if(credentials.is_some()){
            let cred: Credential = credentials.unwrap();
            let mut url: String = format!("{}://{}:{}@{}:{}", self.protocol, cred.username, cred.password, self.host, self.port);
            url = self.append_host(url).to_owned();
            println!("{}", url);
            self.create_connection_object(url)
        }else{
            let mut url: String = format!("{}://{}:{}", self.protocol, self.host, self.port);
            url = self.append_host(url).to_owned();
            self.create_connection_object(url)
        }
    }

    /// Create a new object
    fn new(protocol: String, host: String, port: &i64, vhost: Option<String>) -> RabbitMQConnectionFactory{
        RabbitMQConnectionFactory{
            protocol: protocol,
            host: host,
            port: port.to_owned(),
            vhost: vhost
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::DerefMut;

    #[test]
    fn should_create_new(){
        let f = RabbitMQConnectionFactory::new(String::from("amqp"), String::from("127.0.0.1"), &5672, Some(String::from("test")));
        let cred = Credential{
            username: String::from("dev"),
            password: String::from("rtp*4500"),
        };
        let conn_object = f.create_connection(Some(cred), false);
        let conn = conn_object.unwrap();
        assert!(conn.is_some());
        let copt = conn.to_owned();
        let m = copt.unwrap();
        let a = m.lock().unwrap();
    }
}
