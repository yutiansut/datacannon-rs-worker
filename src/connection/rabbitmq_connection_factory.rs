/// A rabbit mq connection factory for obtaining rabbit mq based connections.
/// Author: Andrew Evans
use std::fmt;
use std::sync::{Arc, Mutex};
use crate::connection::RabbitMQConnection;
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
    fn create_connection_object(self, url: String) -> Result<Option<Arc<Mutex<RabbitMQConnection>>>, Err> {
        let conn_result = Connection::open(url.as_str());
        if(conn_result.is_ok()){
            let mut conn = conn_result.unwrap();
            let channel_result = conn.open_channel(None);
            if(channel_result.is_ok()) {
                let channel = channel_result.unwrap();
                let conn_object = RabbitMQConnection {
                    connection: conn,
                    channel: channel,
                };
                Some(Arc::new(Mutex::new(conn_object)))
            }else{
                Err("Failed to Establish a Channel")
            }
        }else {
            Err("Failed to Establish a Connection")
        }
    }

    /// Append a host if it exists
    fn append_host(self, in_url: String) -> String{
        let mut url = in_url.clone();
        if(self.vhost.is_some()){
            let host: String = self.vhost.unwrap();
            url = format!("{}/{}", url, host);
        }
        url
    }

    /// Create a RabbitMQ Connection
    fn create_connection(self, credentials: Option<Credential>, is_ssl: bool) -> Result<Option<Arc<Mutex<RabbitMQConnection>>>,Err> {
        if(credentials.is_some()){
            let cred: Credential = credentials.unwrap();
            let mut url: String = format!("{}::/{}:{}@{}:{}", self.protocol, cred.username, cred.password, self.host, self.port);
            url = self.append_host(url).to_owned();
            self.create_connection_object(url)
        }else{
            let mut url: String = format!("{}::/{}:{}", self.protocol, self.host, self.port);
            url = self.append_host(url).to_owned()
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

    #[test]
    fn should_create_new(){

    }

    #[text]
    fn should_create_an_exchange_if_needed(){

    }

    #[test]
    fn test_should_create_connection_and_channel(){

    }
}
