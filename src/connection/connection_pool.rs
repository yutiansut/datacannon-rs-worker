/*
Connection Pool for RabbitMQ connection pooling

author Andrew Evans
*/

use std::vec::Vec;
use std::sync::{Arc, Mutex, RwLock};

use crate::queue::amqp::AMQPConnectionInf;
use crate::queue::rabbitmq_utils::QueueUtils;
use std::borrow::Borrow;


/// Structure storing the pool
pub struct RabbitMQConnectionPool{
    size: usisze,
    connections: Vec<Arc<RwLock<QueueUtils>>>,
    checked_out_connections: Vec<Arc::new(Mutex::new(QueueUtils))>,
}


/// Structure implementation
impl RabbitMQConnectionPool{

    /// Add a connection to the pool
    fn add_connection(&self, conn_inf: AMQPConnectionInf) -> Result<(), &'static str>{
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
            Ok(())
        }else{
            Err("Failed to Establish Connection")
        }
    }

    /// Drop the connection
    fn drop_connection(&mut self, idx: &usize){
        if idx < self.connections.len() {
            let mut new_vec = Vec::<Arc<RwLock<QueueUtils>>>::new();
            for i in 0..self.connections.len() {
                let qconn = self.connections.pop().unwrap();
                if &i != idx {
                    new_vec.push(qconn);
                } else {
                    let r = qconn.close_conn();
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
            let q = self.connections.pop();
            q.unwrap().close_conn();
        }
    }

    fn get_connection(&self){

    }

    fn release_connection(&self){

    }

    fn start(&self){

    }

    /// Create a new pool
    fn new(size: usize, conn_inf: AMQPConnectionInf) -> RabbitMQConnectionPool{
        let v = Vec::<Arc<RwLock<QueueUtils>>>::new();
        let checked_out = Vec::<Arc<RwLock<QueueUtils>>>::new();
        RabbitMQConnectionPool{
            size: size,
            connections: v,
            checked_out_connections: checked_out,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn should_create_pool(){

    }

    #[test]
    fn should_get_connection(){

    }

    #[test]
    fn should_release_and_reset_dead_connection(){

    }

    #[test]
    fn should_release_connection(){

    }

    #[test]
    fn should_close_pool(){

    }
}
