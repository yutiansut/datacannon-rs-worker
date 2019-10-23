/*
Connection Pool for RabbitMQ connection pooling

author Andrew Evans
*/

use std::vec::Vec;

use crate::queue::amqp::AQMPConnectionInf;
use crate::queue::rabbitmq_utils::QueueUtils;


pub struct RabbitMQConnectionPool{
    connections: Vec<QueueUtils>,
}


impl RabbitMQConnectionPool{

    fn add_connection(&self, conn_inf: AQMPConnectionInf){

    }

    fn get_pool_size(&self) -> usize{
        let conns = self.connections;
        9
    }

    fn close(&self){

    }

    fn new(nconnections: &i8) -> RabbitMQConnectionPool{

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
    fn should_close_pool(){

    }
}
