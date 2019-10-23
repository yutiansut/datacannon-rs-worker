/*
Connection Pool for RabbitMQ connection pooling

author Andrew Evans
*/

use std::borrow::Borrow;
use std::sync::{Arc, RwLock};
use std::vec::Vec;

use crate::queue::amqp::AMQPConnectionInf;
use crate::queue::rabbitmq_utils::QueueUtils;

/// Structure storing the pool
pub struct RabbitMQConnectionPool{
    initial_size: usize,
    connections: Vec<Box<RwLock<QueueUtils>>>,
}


/// Structure implementation
impl RabbitMQConnectionPool{

    /// Add a connection to the pool
    fn add_connection(&mut self, conn_inf: AMQPConnectionInf) -> Result<(), &'static str>{
        let protocol = conn_inf.get_protocol();
        let host = conn_inf.get_host();
        let port = conn_inf.get_port();
        let vhost = conn_inf.get_vhost();
        let username = conn_inf.get_username();
        let password = conn_inf.get_password();
        let is_ssl = conn_inf.get_ssl();
        let r = QueueUtils::new(protocol, host, &port, vhost, username, password, is_ssl);
        if(r.is_ok()){
            let qutils = r.ok().unwrap();
            self.connections.push(Box::new(RwLock::new(qutils)));
            Ok(())
        }else{
            Err("Failed to Establish Connection")
        }
    }

    /// Drop the connection
    fn drop_connection(&mut self, idx: &usize){
        if idx < &self.connections.len() {
            let mut new_vec = Vec::<Box<RwLock<QueueUtils>>>::new();
            for i in 0..self.connections.len() {
                let mut qconn = self.connections.pop().unwrap();
                if &i != idx {
                    new_vec.push(qconn);
                } else {
                    let r = qconn.into_inner().ok().unwrap().close_conn();
                }
            }
            self.connections = new_vec;
        }
    }

    /// Get the pool size
    fn get_pool_size(&self) -> usize{
        self.connections.len()
    }

    /// Close the connection pool
    fn close(&mut self){
        for i in 0..self.connections.len(){
            let mut qconn = self.connections.pop().unwrap();
            let r = qconn.into_inner().ok().unwrap().close_conn();
        }
    }

    /// Obtain a connection that must be released back to the pool
    fn get_connection(&mut self) -> Result<Box<RwLock<QueueUtils>>, &'static str>{
        if self.connections.len() > 0{
            let conn = self.connections.pop();
            let qbox = conn.unwrap();
            Ok(qbox)
        }else{
            Err("No Connections")
        }
    }

    /// Release a connection
    fn release_connection(&mut self, connection: Box<RwLock<QueueUtils>>){
        self.connections.push(connection);
    }

    /// Start the pool
    fn start(&mut self, conn_inf: AMQPConnectionInf){
        for i in 0..self.initial_size{
            self.add_connection(conn_inf.clone());
        }
    }

    /// Create a new pool
    fn new(size: usize) -> RabbitMQConnectionPool{
        let v = Vec::<Box<RwLock<QueueUtils>>>::new();
        RabbitMQConnectionPool{
            initial_size: size,
            connections: v,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::sync::{LockResult, Mutex, PoisonError};
    use amiquip::{ExchangeDeclareOptions, ExchangeType, FieldTable, QueueDeclareOptions, QueueDeleteOptions};
    use crossbeam::thread;
    use std::borrow::BorrowMut;

    fn get_amqp_conn_inf() -> AMQPConnectionInf {
        AMQPConnectionInf{
            protocol: String::from("amqp"),
            host: String::from("127.0.0.1"),
            port: 5672,
            vhost: Some(String::from("test")),
            username: Some(String::from("dev")),
            password: Some(String::from("rtp*4500")),
            is_ssl: false
        }
    }

    #[test]
    fn should_start_pool(){
        let conn_inf  = get_amqp_conn_inf();
        let mut pool = RabbitMQConnectionPool::new(3);
        pool.start(conn_inf);
        pool.close();
    }

    #[test]
    fn should_close_pool(){
        let conn_inf  = get_amqp_conn_inf();
        let mut pool = RabbitMQConnectionPool::new(3);
        pool.start(conn_inf);
        pool.close();
    }

    #[test]
    fn should_add_more_connections(){
        let conn_inf  = get_amqp_conn_inf();
        let mut pool = RabbitMQConnectionPool::new(3);
        pool.start(conn_inf.clone());
        for i in 0..2{
            pool.add_connection(conn_inf.clone());
        }
        pool.close();
    }

    #[test]
    fn should_get_connection(){
        let conn_inf  = get_amqp_conn_inf();
        let mut pool = RabbitMQConnectionPool::new(3);
        pool.start(conn_inf);
        let conn_res = pool.get_connection();
        let conn_box = conn_res.ok();
        conn_box.unwrap().into_inner().unwrap().close_conn();
        pool.close();
    }

    #[test]
    fn should_release_connection(){
        let conn_inf  = get_amqp_conn_inf();
        let mut pool = RabbitMQConnectionPool::new(3);
        pool.start(conn_inf);
        for i in 0..6 {
            let conn_res = pool.get_connection();
            let conn_box = conn_res.ok().unwrap();
            pool.release_connection(conn_box);
        }
        pool.close();
    }

    #[test]
    fn should_perform_function(){
        let conn_inf  = get_amqp_conn_inf();
        let mut pool = RabbitMQConnectionPool::new(3);
        pool.start(conn_inf);
        let qname = String::from("test");
        let qopts = QueueDeclareOptions::default();
        let conn_res = pool.get_connection();
        let mut qutils = conn_res.ok().unwrap();
        let pq = Arc::new(Mutex::new(qutils));
        let mut pref = Arc::new(Mutex::new(pool));
        let t = thread::scope(|s| {
            s.spawn(move |_| {
                pq.clone().lock().unwrap().get_mut().unwrap().create_queue(qname, qopts, false);
            });
        }).unwrap();
        let qutils = pq.clone().lock().ok().unwrap().get_mut().unwrap();
        //pref.clone().lock().unwrap().release_connection(qutils.);
        pref.lock().unwrap().initial_size;
    }
}
