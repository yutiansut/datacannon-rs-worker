/*
Queue utilities for client and worker

Author Andrew Evans
*/

use crate::connection::rabbitmq_connection_utils::RabbitMQConnection;
use amiquip::{ExchangeType, ExchangeDeclareOptions, QueueDeclareOptions, FieldTable, QueueDeleteOptions};
use std::sync::{Mutex, Arc};
use std::borrow::{BorrowMut, Borrow};
use crate::connection::rabbitmq_connection_utils;
use crate::connection::rabbitmq_connection_factory::{RabbitMQConnectionFactory, Credential};
use std::ops::{Deref, DerefMut};


pub struct QueueUtils{
    connection: RabbitMQConnection,
}


impl QueueUtils{

    /// Create exchange if not exists
    pub fn create_exchange(&self, exchange_name: String, exchange_type: ExchangeType, exchange_options: ExchangeDeclareOptions){
        self.connection.channel.exchange_declare(exchange_type, exchange_name, exchange_options);
    }

    ///create an exchange with default options
    pub fn create_default_exchange(&self, exchange_name: String, exchage_type: ExchangeType){
        self.connection.channel.exchange_declare(exchage_type, exchange_name, ExchangeDeclareOptions::default());
    }

    /// Drop the exchange
    pub fn drop_exchange(&self, exchange_name: String, if_unused: bool){
        self.connection.channel.exchange_delete(exchange_name, if_unused);
    }

    ///force drop the exchage
    pub fn force_drop_exchange(&self, exchange_name: String){
        self.connection.channel.exchange_delete(exchange_name, true);
    }

    /// bind to exchange
    pub fn exchange_bind(&self, source: String, destination: String, routing_key: String, fieldTable: FieldTable){
        self.connection.channel.exchange_bind(destination, source, routing_key, fieldTable);
    }

    /// Create a queue if not exists
    pub fn create_queue(&self, queue_name: String, options: QueueDeclareOptions, nowait: bool){
        if(nowait == true) {
            self.connection.channel.queue_declare_nowait(queue_name, options);
        }else{
            self.connection.channel.queue_declare(queue_name, options);
        }
    }

    ///Bind to a queue
    pub fn bind_queue(&self, queue_name: String, exchange: String, routing_key: String, fieldTable: FieldTable, nowait: bool){
        if(nowait == true){
            self.connection.channel.queue_bind_nowait(queue_name, exchange, routing_key, fieldTable);
        }else{
            self.connection.channel.queue_bind(queue_name, exchange, routing_key, fieldTable);
        }
    }

    /// Drop the queue
    pub fn drop_queue(&self, queue_name: String, options: QueueDeleteOptions, nowait: bool){
        if(nowait == true) {
            self.connection.channel.queue_delete_nowait(queue_name, options);
        }else{
            self.connection.channel.queue_delete(queue_name, options);
        }
    }

    ///Close the attached conection
    pub fn close_conn(self) -> bool{
        let mut result = self.connection.channel.close();
        if(result.is_ok()){
            result = self.connection.connection.close();
            if(result.is_ok()){
                true
            }else{
                false
            }
        }else{
            false
        }
    }

    ///create a new set of utilities
    pub fn new(protocol: String, host: String, port: &i64, vhost: Option<String>, username: Option<String>, password: Option<String>, is_ssl: bool) -> Result<QueueUtils, &'static str>{
        let f = RabbitMQConnectionFactory::new(protocol, host, port, vhost);
        if(username.is_some() && password.is_some()){
            let credentials = Credential{
                username: username.unwrap().clone(),
                password: password.unwrap().clone(),
            };
            let connection_mutex = f.create_connection(Some(credentials), is_ssl);
            if(connection_mutex.is_ok()) {
                let conn = connection_mutex.ok().unwrap();
                let q = QueueUtils {
                    connection: conn,
                };

                Ok(q)
            }else{
                Err("Failed to Establish Connection")
            }
        }else{
            let connection_mutex= f.create_connection(None, is_ssl);
            if(connection_mutex.is_ok()) {
                let conn = connection_mutex.ok().unwrap();
                let q = QueueUtils {
                    connection: conn,
                };
                Ok(q)
            }else{
                Err("Failed to Establishe Connection")
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::rabbitmq_connection_factory::{RabbitMQConnectionFactory, Credential};

    #[test]
    fn should_get_and_close_conn() {
        let client = QueueUtils::new(String::from("amqp"), String::from("127.0.0.1"), &5672, Some(String::from("test")),Some(String::from("dev")), Some(String::from("rtp*4500")), false);
        let q= client.ok().unwrap();
        q.close_conn();
    }

    #[test]
    fn should_create_queue(){
        let client = QueueUtils::new(String::from("amqp"), String::from("127.0.0.1"), &5672, Some(String::from("test")),Some(String::from("dev")), Some(String::from("rtp*4500")), false);
        let q= client.ok().unwrap();
        let mut qopts = QueueDeclareOptions::default();
        qopts.durable = true;
        q.create_queue(String::from("celtest"), qopts, false);
        q.close_conn();
    }

    #[test]
    fn should_bind_to_queue(){
        let client = QueueUtils::new(String::from("amqp"), String::from("127.0.0.1"), &5672, Some(String::from("test")),Some(String::from("dev")), Some(String::from("rtp*4500")), false);
        let q= client.ok().unwrap();
        let mut qopts = QueueDeclareOptions::default();
        qopts.durable = true;
        let ename = String::from("extest");
        let qname = String::from("celtest");
        let rkey = String::from("queueutils.test");
        let ftable = FieldTable::default();
        let mut eopts = ExchangeDeclareOptions::default();
        let qdopts = QueueDeleteOptions::default();
        eopts.durable = true;
        let etype = ExchangeType::Direct;
        q.create_exchange(ename.clone(), etype, eopts);
        q.create_queue(qname.clone(), qopts, false);
        q.bind_queue(qname.clone(), ename.clone(), rkey, ftable, false);
        q.drop_exchange(ename.clone(), true);
        q.drop_queue(qname.clone(), qdopts, true);
        q.close_conn();
    }

    #[test]
    fn should_drop_queue(){
        let client = QueueUtils::new(String::from("amqp"), String::from("127.0.0.1"), &5672, Some(String::from("test")),Some(String::from("dev")), Some(String::from("rtp*4500")), false);
        let q= client.ok().unwrap();
        let qname = String::from("celtest");
        let mut qopts = QueueDeclareOptions::default();
        qopts.durable = true;
        q.create_queue(qname.clone(), qopts, false);
        let mut qdopts = QueueDeleteOptions::default();
        qdopts.if_empty = true;
        q.drop_queue(qname.clone(), qdopts, false);
        q.close_conn();
    }

    #[test]
    fn should_create_exchange(){
        let client = QueueUtils::new(String::from("amqp"), String::from("127.0.0.1"), &5672, Some(String::from("test")),Some(String::from("dev")), Some(String::from("rtp*4500")), false);
        let q= client.ok().unwrap();
        let ename = String::from("extest");
        let mut eopts = ExchangeDeclareOptions::default();
        eopts.durable = true;
        let etype = ExchangeType::Direct;
        q.create_exchange(ename, etype, eopts);
        q.close_conn();
    }

    #[test]
    fn should_drop_exchange(){
        let client = QueueUtils::new(String::from("amqp"), String::from("127.0.0.1"), &5672, Some(String::from("test")),Some(String::from("dev")), Some(String::from("rtp*4500")), false);
        let q= client.ok().unwrap();
        let ename = String::from("extest");
        let mut eopts = ExchangeDeclareOptions::default();
        eopts.durable = true;
        let etype = ExchangeType::Direct;
        q.create_exchange(ename.clone(), etype, eopts);
        q.drop_exchange(ename, false);
        q.close_conn();
    }
}
