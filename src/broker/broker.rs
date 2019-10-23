/*
Implementation of available brokers
*/

use crate::queue::rabbitmq_utils::{QueueUtils};


pub struct RabbitMQBroker{
    qutils: QueueUtils,
}


pub trait AMQPBroker{
    fn create_connection(&self, protocol: String, host: String, port: &i64, vhost: String);
    fn package_message(&self, message: String);
    fn send_message(&self);
    fn close_connection(&self);
}


impl AMQPBroker for RabbitMQBroker{

    fn create_connection(&self, protocol: String, host: String, port: &i64, vhost: String){

    }

    fn package_message(&self, message: String){

    }

    fn send_message(&self){

    }

    fn close_connection(&self){

    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn should_create_rabbit_mq_broker(){

    }
}
