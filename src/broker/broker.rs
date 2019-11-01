/*
Implementation of available brokers

Author Andrew Evans
*/

use std::any::Any;
use std::collections::BTreeMap;
use std::env::Args;
use std::iter::Map;

use amiquip::{AmqpProperties, AmqpValue, Channel, Publish, QueueDeclareOptions, Queue, ExchangeDeclareOptions, Exchange, ExchangeType, FieldTable};
use serde_json::Value;

use crate::argparse::kwargs::KwArgs;
use crate::broker::headers::Headers;
use crate::broker::message_body::MessageBody;
use crate::broker::properties::Properties;
use crate::config::config::{CeleryConfig};
use crate::connection::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
use crate::connection::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;
use crate::queue::amqp::AMQPConnectionInf;
use crate::task::config::TaskConfig;
use std::error::Error;

/// RabbitMQ Broker
pub struct RabbitMQBroker{
    config: CeleryConfig,
}


/// AMQP Broker
pub trait AMQPBroker{

    /// bind queue to the exchange
    fn bind_to_exchange(channel: Channel, exchange: String, queue: String, routing_key: String);

    /// create a queue
    fn create_queue(channel: Channel, durable: bool, queue: String, declare_exchange: bool, uuid: String, exchange: Option<String>);

    /// create an exchange
    fn create_exchange(channel: Channel, durable: bool, exchange: String, exchange_type: ExchangeType);

    /// send task to the broker
    fn send_task(channel: Channel, task: String, props: Properties, headers: Headers, body: MessageBody) -> Result<String, ()>;

}


/// AMQP Broker
impl AMQPBroker for RabbitMQBroker{

    /// create the exchange
    fn create_exchange<'a>(channel: Channel, durable: bool, exchange: String, exchange_type: ExchangeType) {
        let mut opts = ExchangeDeclareOptions::default();
        opts.durable = durable;
        let r = channel.exchange_declare(exchange_type, exchange, opts);
    }

    /// create a queue
    fn create_queue(channel: Channel, durable: bool, queue: String, declare_exchange: bool, uuid: String, exchange: Option<String>) {
        let mut qopts = QueueDeclareOptions::default();
        if durable {
            qopts.durable = durable;
        }
        channel.queue_declare(queue, qopts);
    }

    /// bind a queue to an exchange
    fn bind_to_exchange(channel: Channel, exchange: String, queue: String, routing_key: String) {
        let args = FieldTable::new();
        channel.queue_bind(queue, exchange, routing_key, args);
    }

    /// send a task to the broker
    fn send_task(channel: Channel, task: String, props: Properties, headers: Headers, body: MessageBody) -> Result<String, ()> {
        unimplemented!()
    }
}


/// Rabbit MQ broker
impl RabbitMQBroker{

    /// Create a new broker
    pub fn new(config: CeleryConfig) -> RabbitMQBroker{
        RabbitMQBroker{
            config: config.clone(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::config::config::CeleryConfig;
    use crate::queue::amqp::AMQPConnectionInf;

    #[test]
    fn should_create_queue(){
        //let broker_conn = AMQPConnectionInf::new();
        //let conf = CeleryConfig::new(broker_conn, );
    }

    #[test]
    fn should_create_and_bind_queue_to_exchange(){

    }

    #[test]
    fn should_send_task_to_queue(){

    }
}
