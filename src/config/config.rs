/// The configuration object
/// Author: Andrew Evans

use amiquip::ExchangeType;
use num_cpus;
use std::collections::HashMap;
use crate::amqp::amqp::AMQPConnectionInf;
use crate::backend::backend::Backend;


#[derive(Clone, Debug)]
pub enum QueuePersistenceType{
    PERSISTENT,
    NONPERSISTENT
}


#[derive(Clone, Debug)]
pub enum BackendType{
    RPC,
    REDIS
}


#[derive(Clone, Debug)]
pub enum BrokerType{
    RABBITMQ,
}


#[derive(Clone, Debug)]
pub struct Admin{
    name: String,
    email: String,
}


#[derive(Clone, Debug)]
pub struct CeleryConfig{
    pub connection_inf: AMQPConnectionInf,
    pub broker_connection_retry: bool,
    pub result_backend: Backend,
    pub celery_cache_backend: Option<String>,
    pub send_events: bool,
    pub queues: Vec<String>,
    pub default_exchange_type: ExchangeType,
    pub default_queue: String,
    pub broadcast_exchange: String,
    pub broadcast_exchange_type: ExchangeType,
    pub event_queue: String,
    pub event_exchange: String,
    pub event_exchange_type: ExchangeType,
    pub event_routing_key: String,
    pub event_serializer: String,
    pub result_exchange: String,
    pub accept_content: String,
    pub worker_prefetch_multiplier: i8,
    pub default_delivery_mode: QueuePersistenceType,
    pub default_routing_key: String,
    pub broker_connection_timeout: i64,
    pub broker_connection_max_retries: i64,
    pub broadcast_queue: String,
    pub backend_type: BackendType,
    pub broker_type: BrokerType,
    pub celery_send_task_error_emails: bool,
    pub admins: Vec<Admin>,
    pub server_email: String,
    pub mail_host: String,
    pub mail_host_user: Option<String>,
    pub mail_host_password: Option<String>,
    pub mail_port: i8,
    pub always_eager: bool,
    pub eager_propogates_exceptions: bool,
    pub track_started: bool,
    pub acks_late: bool,
    pub store_errors_even_if_ignored: bool,
    pub task_result_expires: i64,
    pub ignore_result: bool,
    pub max_cached_results: i32,
    pub result_persistent: QueuePersistenceType,
    pub result_serializer: String,
    pub database_engine_options: Option<HashMap<String, String>>,
    pub default_rate_limit: i8,
    pub disable_rate_limits: bool,
    pub celerybeat_log_level: String,
    pub celerybeat_log_file: Option<String>,
    pub celerybeat_schedule_file_name: String,
    pub celerybeat_max_loop_interval: i64,
    pub celerymon_log_level: String,
    pub celerymon_log_file: Option<String>,
    pub num_connections: usize,
}

impl CeleryConfig{

    pub fn new(broker_inf: AMQPConnectionInf, backend: Backend) -> CeleryConfig{
        CeleryConfig{
            connection_inf: broker_inf,
            broker_connection_retry: true,
            result_backend: backend,
            celery_cache_backend: None,
            send_events: false,
            queues: Vec::<String>::new(),
            default_exchange_type: ExchangeType::Direct,
            default_queue: String::from("celery"),
            broadcast_exchange: String::from("celeryctl"),
            broadcast_exchange_type: ExchangeType::Fanout,
            event_queue: String::from("celeryevent"),
            event_exchange: String::from("celery_event"),
            event_exchange_type: ExchangeType::Topic,
            event_routing_key: String::from("celeryevent"),
            event_serializer: String::from("json"),
            result_exchange: String::from("celeryresult"),
            accept_content: String::from("application/json"),
            worker_prefetch_multiplier: 4,
            default_delivery_mode: QueuePersistenceType::PERSISTENT,
            default_routing_key: String::from("celery"),
            broker_connection_timeout: 10000,
            broker_connection_max_retries: 1000,
            broadcast_queue: String::from("celeryctl"),
            backend_type: BackendType::RPC,
            broker_type: BrokerType::RABBITMQ,
            celery_send_task_error_emails: false,
            admins: Vec::<Admin>::new(),
            server_email: String::from("celery@localhost"),
            mail_host: String::from("localhost"),
            mail_host_user: None,
            mail_host_password: None,
            mail_port: 25,
            always_eager: false,
            eager_propogates_exceptions: true,
            track_started: false,
            acks_late: true,
            store_errors_even_if_ignored: false,
            task_result_expires: 600000,
            ignore_result: false,
            max_cached_results: 100,
            result_persistent: QueuePersistenceType::NONPERSISTENT,
            result_serializer: String::from("json"),
            database_engine_options: None,
            default_rate_limit: -1,
            disable_rate_limits: true,
            celerybeat_log_level: String::from("INFO"),
            celerybeat_log_file: None,
            celerybeat_schedule_file_name: String::from("celerybeat-schedule"),
            celerybeat_max_loop_interval: 300000,
            celerymon_log_level: String::from("INFO"),
            celerymon_log_file: None,
            num_connections: num_cpus::get(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_configuration(){
        let broker_conf = AMQPConnectionInf::new(
            String::from("amqp"),
            String::from("127.0.0.1"),
            5672,
            Some(String::from("test")),
            Some(String::from("dev")),
            Some(String::from("rtp*4500")),
            false
        );
        let b = Backend{};
        let c = CeleryConfig::new(broker_conf, b);
        let url = c.connection_inf.to_url();
        assert!(url.eq("amqp://dev:rtp*4500@127.0.0.1:5672/test"))
    }
}
