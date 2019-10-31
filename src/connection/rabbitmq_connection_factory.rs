/// A rabbit mq connection factory for obtaining rabbit mq based connections.
/// Author: Andrew Evans
use std::fmt;
use std::sync::{Arc, Mutex};

use amiquip::{Channel, Connection};

use crate::connection::rabbitmq_connection_utils;
use crate::connection::rabbitmq_connection_utils::RabbitMQConnection;
use crate::connection::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;
use crate::queue::amqp::AMQPConnectionInf;

///Credentials object
pub struct Credential{
    pub username: String,
    pub password: String,
}


///Overarching Rabbit MQ Connection Factory
pub struct RabbitMQConnectionFactory{
    conn_inf: AMQPConnectionInf,
}


///Implementation of the Rabbit MQ Connection Factory
impl RabbitMQConnectionFactory {

    /// Open a connection and channel. Return a connection object with blocking access
    fn create_connection_object(&self, url: String) -> Result<RabbitMQConnection, &'static str> {
        RabbitMQConnection::new(url)
    }

    /// create a threadable connection object
    fn create_threadable_connection_object(&self, url: String) -> Result<ThreadableRabbitMQConnection, &'static str> {
        ThreadableRabbitMQConnection::new(url)
    }

    /// Create a RabbitMQ Connection
    pub fn create_connection(&self, credentials: Option<Credential>, is_ssl: bool) -> Result<RabbitMQConnection, &'static str> {
        if(credentials.is_some()){
            let url = self.conn_inf.to_url();
            self.create_connection_object(url)
        }else{
            let url = self.conn_inf.to_url();
            self.create_connection_object(url)
        }
    }


    /// Create a thread safe connection from the factory
    pub fn create_threadable_connection(&self) -> Result<ThreadableRabbitMQConnection, &'static str> {
        let url= self.conn_inf.to_url();
        self.create_threadable_connection_object(url)
    }

    /// Create a new object
    pub fn new(conn_inf: AMQPConnectionInf) -> RabbitMQConnectionFactory{
        RabbitMQConnectionFactory{
            conn_inf: conn_inf,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::ops::DerefMut;
    use std::thread;

    use crate::connection::rabbitmq_connection_utils;

    use super::*;

    #[test]
    fn should_create_new(){
        let aci = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 3030, None, None, None, false);
        let f = RabbitMQConnectionFactory::new(aci);
        let cred = Credential{
            username: String::from("dev"),
            password: String::from("rtp*4500"),
        };
        let conn_object = f.create_connection(Some(cred), false).ok().unwrap();
        conn_object.channel.close();
        conn_object.connection.close();
    }
}
