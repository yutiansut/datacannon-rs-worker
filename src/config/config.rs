/// The configuration object
/// Author: Andrew Evans

use num_cpus;
use std::collections::HashMap;


pub enum QueuePersistenceType{
    PERSISTENT,
    NONPERSISTENT
}

pub enum BackendType{
    RPC,
    REDIS
}


pub enum BrokerType{
    RABBITMQ,
}


pub enum ExchangeType{
    DIRECT,
    FANOUT,
    TOPIC,
    HEADERS,
}


pub struct Admin{
    name: String,
    email: String,
}


pub struct CeleryConfig{
    broker_connection_retry: bool,
    result_backend: String,
    celery_cache_backend: Option<String>,
    send_events: bool,
    queues: Vec<String>,
    default_exchange_type: ExchangeType,
    default_queue: String,
    broadcast_exchange: String,
    broadcast_exchange_type: ExchangeType,
    event_queue: String,
    event_exchange: String,
    event_exchange_type: ExchangeType,
    event_routing_key: String,
    event_serializer: String,
    result_exchange: String,
    accept_content: String,
    worker_prefetch_multiplier: i8,
    default_delivery_mode: QueuePersistenceType,
    default_routing_key: String,
    broker_connection_timeout: i64,
    broker_connection_max_retries: i64,
    broadcast_queue: String,
    backend_type: BackendType,
    broker: String,
    broker_type: BrokerType,
    celery_send_task_error_emails: bool,
    admins: Vec<Admin>,
    server_email: String,
    mail_host: String,
    mail_host_user: Option<String>,
    mail_host_password: Option<String>,
    mail_port: i8,
    always_eager: bool,
    eager_propogates_exceptions: bool,
    track_started: bool,
    acks_late: bool,
    store_errors_even_if_ignored: bool,
    task_result_expires: i64,
    ignore_result: bool,
    max_cached_results: i32,
    result_persistent: QueuePersistenceType,
    result_serializer: String,
    database_engine_options: Option<HashMap<String, String>>,
    default_rate_limit: i8,
    disable_rate_limits: bool,
    celerybeat_log_level: String,
    celerybeat_log_file: Option<String>,
    celerybeat_schedule_file_name: String,
    celerybeat_max_loop_interval: i64,
    celerymon_log_level: String,
    celerymon_log_file: Option<String>,
    num_threads: usize,
}

impl CeleryConfig{

    pub fn new(broker: String, backend: String) -> CeleryConfig{
        CeleryConfig{
            broker_connection_retry: true,
            result_backend: backend,
            celery_cache_backend: None,
            send_events: false,
            queues: Vec::<String>::new(),
            default_exchange_type: ExchangeType::DIRECT,
            default_queue: String::from("celery"),
            broadcast_exchange: String::from("celeryctl"),
            broadcast_exchange_type: ExchangeType::FANOUT,
            event_queue: String::from("celeryevent"),
            event_exchange: String::from("celery_event"),
            event_exchange_type: ExchangeType::TOPIC,
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
            broker: broker,
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
            num_threads: num_cpus::get(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_configuration(){
        let c = CeleryConfig::new(String::from("test"), String::from("test"));
        assert!(c.broker.eq("test"));
        assert!(c.result_backend.eq("test"));
    }
}
