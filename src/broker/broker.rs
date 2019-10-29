/*
Implementation of available brokers

Author Andrew Evans
*/

use std::any::Any;
use std::iter::Map;

use amiquip::{AmqpProperties, AmqpValue};

use crate::config::config::CeleryConfig;
use crate::connection::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;
use crate::message::broker::BrokerMessage;
use crate::message::task_message::TaskMessagev2;
use crate::task::config::TaskConfig;
use std::collections::BTreeMap;


/// RabbitMQ Broker
pub struct RabbitMQBroker{
    config: CeleryConfig
}


/// AMQP Broker
pub trait AMQPBroker{
    //fn get_headers(&self, lang: String, correlation_id: String, task_name: String, expires: String, time_limit: String, soft_time_limit: String, args: Vec<Any>, kwargs: Map<String, &Any>) -> Map<String, String>;
    //fn get_body(&self, args: Vec<Any>, kwargs: Map<String, &Any>) -> Vec<Any>;
    //fn get_body_task_config(&self) -> Map<String, String>;
    //fn get_basic_properties(&self, task_name: String, reply_to: String, correlation_id: String, expires: String, priority: i8, time_limit: i64, soft_time_limit: i64, args: String, kwargs: String, eta: String, retries: i8) -> AmqpProperties;
    //fn create_queues(&self, declare_exchange: bool, uuid: String) -> String;
    //fn send_task_message(&self, connection: ThreadableRabbitMQConnection, message: BrokerMessage, routing_key: String, properties: AmqpProperties);
    //fn package_message(&self, task_name: String, args: Vec<Any>, kwargs: Map<String, &Any>, reply_to: String, correlation_id: String, expires: String, priority: i8, time_limit: i64, soft_time_limit: i64, eta: String, retries: i8) -> TaskMessagev2;
    //fn send_task(&self, task_config: TaskConfig, args: Vec<Any>, kwargs: Map<String, &Any>);
}


/// AMQP Broker
impl AMQPBroker for RabbitMQBroker{

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
