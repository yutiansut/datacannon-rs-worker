/*
Utilities for handling a thread safe rabbitmq connection.

Author Andrew Evans
*/

use amiquip::{ExchangeType, ExchangeDeclareOptions, QueueDeclareOptions, FieldTable, QueueDeleteOptions, Channel};
use crate::connection::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;


/// Create the exchange from an existing channel
fn create_exchange(channel: Channel,  exchange_name: String, exchange_type: ExchangeType, exchange_options: ExchangeDeclareOptions){
    channel.exchange_declare(exchange_type, exchange_name, exchange_options);
}


///create an exchange with default options
pub fn create_default_exchange(channel: Channel, exchange_name: String, exchage_type: ExchangeType){
    channel.exchange_declare(exchage_type, exchange_name, ExchangeDeclareOptions::default());
}

/// Drop the exchange
pub fn drop_exchange(channel: Channel, exchange_name: String, if_unused: bool){
    channel.exchange_delete(exchange_name, if_unused);
}

///force drop the exchage
pub fn force_drop_exchange(channel: Channel, exchange_name: String){
    channel.exchange_delete(exchange_name, true);
}

/// bind to exchange
pub fn exchange_bind(channel: Channel, source: String, destination: String, routing_key: String, fieldTable: FieldTable){
    channel.exchange_bind(destination, source, routing_key, fieldTable);
}

/// Create a queue if not exists
pub fn create_queue(channel: Channel, queue_name: String, options: QueueDeclareOptions, nowait: bool){
    if(nowait == true) {
        channel.queue_declare_nowait(queue_name, options);
    }else{
        channel.queue_declare(queue_name, options);
    }
}

///Bind to a queue
pub fn bind_queue(channel: Channel, queue_name: String, exchange: String, routing_key: String, field_table: FieldTable, nowait: bool){
    if(nowait == true){
        channel.queue_bind_nowait(queue_name, exchange, routing_key, field_table);
    }else{
        channel.queue_bind(queue_name, exchange, routing_key, field_table);
    }
}

/// Drop the queue
pub fn drop_queue(channel: Channel, queue_name: String, options: QueueDeleteOptions, nowait: bool){
    if(nowait == true) {
        channel.queue_delete_nowait(queue_name, options);
    }else{
        channel.queue_delete(queue_name, options);
    }
}
