/*
Implementation of available brokers

Author Andrew Evans
*/

use std::any::Any;
use std::collections::BTreeMap;
use std::env::Args;
use std::iter::Map;

use amiquip::{AmqpProperties, AmqpValue};
use serde_json::Value;

use crate::argparse::kwargs::KwArgs;
use crate::broker::headers::Headers;
use crate::broker::message_body::MessageBody;
use crate::config::config::CeleryConfig;
use crate::connection::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;
use crate::task::config::TaskConfig;
use crate::broker::properties::Properties;
use crate::queue::amqp::AMQPConnectionInf;

/// RabbitMQ Broker
pub struct RabbitMQBroker{
    config: CeleryConfig,
    connection_pool: ThreadableRabbitMQConnection,
}


/// AMQP Broker
pub trait AMQPBroker{
    fn send_task(&self, task: String, props: Properties, headers: Headers, body: MessageBody) -> Result<String, ()>;
    fn bind_to_exchange(&self, exchange: String, queue: String) -> Result<String, ()>;
    fn create_queue(&self, declare_exchange: bool, uuid: String) -> Result<String, ()>;
}


/// AMQP Broker
impl AMQPBroker for RabbitMQBroker{
    fn send_task(&self, task: String, props: Properties, headers: Headers, body: MessageBody) -> Result<String, ()> {
        unimplemented!()
    }

    fn bind_to_exchange(&self, exchange: String, queue: String) -> Result<String, ()> {
        unimplemented!()
    }

    fn create_queue(&self, declare_exchange: bool, uuid: String) -> Result<String, ()> {
        unimplemented!()
    }
}


/// Rabbit MQ broker
impl RabbitMQBroker{

    fn setup_connection_pool(config: CeleryConfig) -> ThreadableRabbitMQConnection{

        unimplemented!()
    }

    /// Create a new broker
    pub fn new(config: CeleryConfig) -> RabbitMQBroker{
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn should_package_headers(){

    }

    #[test]
    fn should_package_body(){

    }

    #[test]
    fn should_get_body_task_config(){

    }

    #[test]
    fn should_get_basic_properties(){

    }

    #[test]
    fn should_create_queues(){

    }

    #[test]
    fn should_send_task_message(){

    }

    #[test]
    fn should_package_message(){

    }

    #[test]
    fn should_send_task(){

    }
}
