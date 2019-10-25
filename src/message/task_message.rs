/*
Task message to send to the broker. Stores properties and other configuration
that the user might not need to know about.

Author Andrew Evans
*/

use amiquip::AmqpProperties;
use std::any::Any;


/// Celery based task message
pub struct TaskMessagev2{
    queue_name: String,
    task: String,
    exchange_name: String,
    routing_key: String,
    properties: AmqpProperties,
    reply_to: String,
    correlation_id: String,
}
