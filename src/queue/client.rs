/// Client queue utilities
/// author Andrew Evans

use crate::connection::rabbitmq_connection_utils::RabbitMQConnection;
use redis::Client;
use std::borrow::Borrow;
use amiquip::ExchangeType;


struct ClientQueueUtils{
    connection: RabbitMQConnection
}


impl ClientQueueUtils{

    /// Create exchange if not exists
    pub fn create_exchange(&self, exchange_name: String){
        //self.connection.borrow().channel.exchange_declare()
    }

    /// Create a queue if not exists
    pub fn create_queue(&self, exchange_name: String, queue_name: String){

    }

    ///create a new set of utilities
    pub fn new(connection: RabbitMQConnection) -> ClientQueueUtils{
        ClientQueueUtils{
            connection: connection,
        }
    }
}
