/*
A result message for handling responses.

Author Andrew Evans
*/

use std::any::Any;


/// Result Message
struct ResultMessagev2{
    queue_name: String,
    task: String,
    exchange_name: String,
    routing_key: String,
    body_objects: Vec<Any>,
}
