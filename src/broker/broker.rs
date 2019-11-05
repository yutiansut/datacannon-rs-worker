/*
Implementation of available brokers in a non-asynchronous manner.

Author Andrew Evans
*/

use std::any::Any;
use std::collections::BTreeMap;
use std::env::Args;
use std::iter::Map;

use celery_rs_core::amqp::amqp::AMQPConnectionInf;
use celery_rs_core::argparse::kwargs::KwArgs;
use celery_rs_core::amqp::{queue_error::QueueError, publish_error::PublishError, exchange_error::ExchangeError};
use celery_rs_core::message_protocol::{message_body::MessageBody, properties::Properties, headers::Headers, message::Message};
use celery_rs_core::task::config::TaskConfig;
use amiquip::{AmqpProperties, AmqpValue, Channel, Publish, QueueDeclareOptions, Queue, ExchangeDeclareOptions, Exchange, ExchangeType, FieldTable};
use serde_json::{Value, to_string};

use crate::config::config::{CeleryConfig};
use crate::connection::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
use crate::connection::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;
use std::error::Error;
use serde_json::map::Values;


/// RabbitMQ Broker
pub struct RabbitMQBroker{
    config: CeleryConfig,
}


/// AMQP Broker
pub trait AMQPBroker{

    /// bind queue to the exchange
    fn bind_to_exchange(config: CeleryConfig, channel: &Channel, exchange: String, queue: String, routing_key: String) -> Result<bool, ExchangeError>;

    /// create a queue
    fn create_queue(config: CeleryConfig, channel: &Channel, durable: bool, queue: String, declare_exchange: bool, uuid: String, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, QueueError>;

    /// create an exchange
    fn create_exchange(config: CeleryConfig, channel: &Channel, durable: bool, exchange: String, exchange_type: ExchangeType) -> Result<bool, ExchangeError>;

    /// send task to the broker
    fn send_task(config: CeleryConfig, channel: &Channel, props: Properties, headers: Headers, body: MessageBody, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, PublishError>;
}


/// AMQP Broker
impl AMQPBroker for RabbitMQBroker{

    /// create the exchange
    fn create_exchange(config: CeleryConfig, channel: &Channel, durable: bool, exchange: String, exchange_type: ExchangeType) -> Result<bool, ExchangeError> {
        let mut opts = ExchangeDeclareOptions::default();
        opts.durable = durable;
        let r = channel.exchange_declare(exchange_type, exchange, opts);
        if r.is_ok(){
            Ok(true)
        }else{
            Err(ExchangeError)
        }
    }

    /// create a queue
    fn create_queue(config: CeleryConfig, channel: &Channel, durable: bool, queue: String, declare_exchange: bool, uuid: String, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, QueueError>{
        let mut qopts = QueueDeclareOptions::default();
        if declare_exchange{
            let mut etype = ExchangeType::Direct;
            let mut eopts= ExchangeDeclareOptions::default();
            eopts.durable = durable;
            channel.exchange_declare(etype, exchange.clone().unwrap(), eopts);
        }
        if durable {
            qopts.durable = durable;
        }
        let r = channel.queue_declare(queue.clone(), qopts);
        if r.is_ok(){
            //bind queue to exchange
            if exchange.is_some(){
                let exchange_name = exchange.unwrap();
                let args = FieldTable::new();
                let mut m_routing_key = config.default_routing_key.clone();
                if routing_key.is_some(){
                    m_routing_key = routing_key.unwrap();
                }
                let er = channel.queue_bind(queue, exchange_name, m_routing_key, args);
                if er.is_ok(){
                    Ok(true)
                }else{
                    Err(QueueError)
                }
            }else {
                Ok(true)
            }
        }else{
            Err(QueueError)
        }
    }

    /// bind a queue to an exchange
    fn bind_to_exchange(config: CeleryConfig, channel: &Channel, exchange: String, queue: String, routing_key: String) -> Result<bool, ExchangeError> {
        let args = FieldTable::new();
        let r = channel.queue_bind(queue, exchange, routing_key, args);
        if r.is_ok(){
            Ok(true)
        }else{
            Err(ExchangeError)
        }
    }

    /// send a task to the broker
    fn send_task(config: CeleryConfig, channel: &Channel, props: Properties, headers: Headers, body: MessageBody, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, PublishError> {
        let cfg = config.clone();
        let mut amq_properties = props.convert_to_amqp_properties();
        let amq_headers = headers.convert_to_btree_map();
        let json_val = Value::from(body.convert_to_json_map());
        let mut json_message = to_string(&json_val);
        if json_message.is_ok() {
            let mut m_routing_key = cfg.default_routing_key.clone();
            let mut m_exchange = cfg.default_routing_key;
            if exchange.is_some(){
                m_exchange = exchange.unwrap();
            }
            if routing_key.is_some(){
                m_routing_key = routing_key.unwrap();
            }
            amq_properties = amq_properties.with_headers(amq_headers);
            let jmessage = json_message.unwrap();
            let jbytes = jmessage.as_bytes();
            let mut publish = Publish::with_properties(jbytes, m_routing_key, amq_properties);
            channel.basic_publish(m_exchange, publish);
            Ok(true)
        }else{
            let e = PublishError;
            Err(e)
        }
    }
}


/// Rabbit MQ broker
impl RabbitMQBroker{

    /// Start a task
    pub fn start_task(&self) {

    }

    /// Create a new broker
    pub fn new(config: CeleryConfig) -> RabbitMQBroker{
        RabbitMQBroker{
            config: config.clone(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::config::CeleryConfig;
    use celery_rs_core::amqp::amqp::AMQPConnectionInf;
    use crate::backend::backend::Backend;
    use crate::broker::broker::{RabbitMQBroker, AMQPBroker};
    use crate::connection::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
    use uuid::Uuid;
    use amq_protocol::frame::AMQPFrameType::Header;

    fn get_config() -> CeleryConfig {
        let protocol = "amqp".to_string();
        let host = "127.0.0.1".to_string();
        let port = 5672;
        let vhost = Some("test".to_string());
        let username = Some("dev".to_string());
        let password = Some("rtp*4500".to_string());
        let broker_conn = AMQPConnectionInf::new(protocol, host, port, vhost, username, password, false);
        let backend = Backend{};
        let conf = CeleryConfig::new(broker_conn, backend);
        conf
    }

    #[test]
    fn should_create_queue(){
        let conf = get_config();
        let rmq = RabbitMQBroker::new(conf.clone());
        let conn_inf = conf.connection_inf.clone();
        let mut pool = ThreadableRabbitMQConnectionPool::new(conn_inf, 2);
        pool.start();
        let rconn = pool.get_connection();
        if rconn.is_ok(){
            let mut c = rconn.unwrap();
            let channel = c.connection.open_channel(None).unwrap();
            let uuid = format!("{}", Uuid::new_v4());
            let rq = RabbitMQBroker::create_queue(conf.clone(), &channel, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
            c.connection.close();
            assert!(rq.is_ok());
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_create_and_bind_queue_to_exchange(){
        let conf = get_config();
        let rmq = RabbitMQBroker::new(conf.clone());
        let conn_inf = conf.connection_inf.clone();
        let mut pool = ThreadableRabbitMQConnectionPool::new(conn_inf, 2);
        pool.start();
        let rconn = pool.get_connection();
        if rconn.is_ok(){
            let mut c = rconn.unwrap();
            let channel = c.connection.open_channel(None).unwrap();
            let uuid = format!("{}", Uuid::new_v4());
            let rq = RabbitMQBroker::create_queue(conf.clone(), &channel, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
            RabbitMQBroker::bind_to_exchange(conf.clone(), &channel,  "test_exchange".to_string(), "test_queue".to_string(), "test_routing_key".to_string());
            c.connection.close();
            assert!(rq.is_ok());
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_send_task_to_queue(){
        let conf = get_config();
        let rmq = RabbitMQBroker::new(conf.clone());
        let conn_inf = conf.connection_inf.clone();
        let mut pool = ThreadableRabbitMQConnectionPool::new(conn_inf, 2);
        pool.start();
        let rconn = pool.get_connection();
        if rconn.is_ok(){
            let mut c = rconn.unwrap();
            let channel = c.connection.open_channel(None).unwrap();
            let uuid = format!("{}", Uuid::new_v4());

            // create queue if necessary
            let rq = RabbitMQBroker::create_queue(conf.clone(), &channel, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));

            // create and send task
            let body = MessageBody::new(None, None, None, None);
            let uuid = Uuid::new_v4();
            let ustr = format!("{}", uuid);
            let headers = Headers::new("rs".to_string(), "test_task".to_string(), ustr.clone(), ustr.clone());
            let reply_queue = Uuid::new_v4();
            let props = Properties::new(ustr.clone(), "application/json".to_string(), "utf-8".to_string(), None);
            let br = RabbitMQBroker::send_task(conf, &channel,props, headers, body, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
            c.connection.close();
            assert!(br.is_ok());
            assert!(rq.is_ok());
        }else{
            assert!(false);
        }
    }
}
