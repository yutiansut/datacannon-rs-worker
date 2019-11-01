/*
Creates and manages broker connections and backend connections

Author Andrew Evans
*/

use crate::connection::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
use crate::broker::broker::RabbitMQBroker;
use crate::config::config::CeleryConfig;
use crate::config::config::BrokerType::RABBITMQ;


/// RabbitMQ client
pub struct RabbitMQClient{
    connection_pool: ThreadableRabbitMQConnectionPool,
    broker: RabbitMQBroker,
}


/// Client trait
pub trait Client{
    fn close(&mut self);
}


/// trait implementation for rabbitmq
impl Client for RabbitMQClient{

    //// Close the connection pool
    fn close(&mut self){
        self.connection_pool.close_pool();
    }
}


/// Client implementation
impl RabbitMQClient{

    /// Create a new client
    pub fn new(config: CeleryConfig) -> RabbitMQClient{
        let pool_config = config.clone();
        let mut pool = ThreadableRabbitMQConnectionPool::new(pool_config.connection_inf.clone(), pool_config.num_connections);
        let broker = RabbitMQBroker::new(config);
        pool.start();
        RabbitMQClient{
            connection_pool: pool,
            broker: broker,
        }
    }
}


#[cfg(test)]
mod tests{
    use crate::client::client::RabbitMQClient;
    use crate::config::config::CeleryConfig;
    use crate::amqp::amqp::AMQPConnectionInf;
    use crate::backend::backend::Backend;

    #[test]
    fn should_create_and_close_client(){
        let broker_connection_inf = AMQPConnectionInf::new("aqmp".to_string(), "127.0.0.1".to_string(), 5672, Some("test".to_string()), Some("dev".to_string()), Some("rtp*4500".to_string()), false);
        let b = Backend{};
        let mut config = CeleryConfig::new(broker_connection_inf, b);
        config.num_connections = 2;
        let client = RabbitMQClient::new(config);
    }
}