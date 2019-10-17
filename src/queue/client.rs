/// Client queue utilities
/// author Andrew Evans

use crate::connection::rabbitmq_connection_utils::RabbitMQConnection;
use redis::Client;
use std::borrow::Borrow;
use amiquip::{ExchangeType, ExchangeDeclareOptions};
use crate::config::config::CeleryConfig;


struct ClientQueueUtils{
    connection: RabbitMQConnection,
}


impl ClientQueueUtils{

    /// Create exchange if not exists
    pub fn create_exchange(&self, exchange_name: String, exchange_type: ExchangeType, exchange_options: ExchangeDeclareOptions){
        self.connection.borrow().channel.exchange_declare(exchange_type, exchange_name, exchange_options);
    }

    ///create an exchange with default options
    pub fn create_default_exchange(&self, exchange_name: String, exchage_type: ExchangeType){
        self.connection.borrow().channel.exchange_declare(exchage_type, exchange_name, ExchangeDeclareOptions::default());
    }

    /// Drop the exchange
    pub fn drop_exchange(&self, exchange_name: String, if_unused: bool){
        self.connection.borrow().channel.exchange_delete(exchange_name, if_unused);
    }

    //force drop the exchage
    pub fn force_drop_exchange(&self, exchange_name: String){
        self.connection.borrow().channel.exchange_delete(exchange_name, true);
    }

    /// bind to exchange
    pub fn exchange_bind(&self){
        self.connection.borrow().channel.exc
    }

    /// Create a queue if not exists
    pub fn create_queue(&self, queue_name: String){

    }

    ///Bind to a queue
    pub fn bind_queue(&self, queue_name: String){

    }

    /// Drop the queue
    pub fn drop_queue(&self, queue_name: String){

    }

    ///create a new set of utilities
    pub fn new(connection: RabbitMQConnection) -> ClientQueueUtils{
        ClientQueueUtils{
            connection: connection,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::DerefMut;

    #[test]
    fn should_create_exchange(){

    }

    #[test]
    fn should_drop_exchange(){

    }

    #[test]
    fn should_create_queue(){

    }

    #[test]
    fn should_drop_queue(){

    }
}
