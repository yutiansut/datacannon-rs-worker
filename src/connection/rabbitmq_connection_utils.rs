/// Handles a connection to rabbit mq.
/// Author: Andrew Evans
use amiquip::{Channel, Connection, Exchange, Publish, Result};
use std::sync::{Arc, Mutex};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;


/// Connection object
pub struct RabbitMQConnection{
    pub connection: Connection,
    pub channel: Channel,
}


/// Implementation
impl RabbitMQConnection {

    /// Create the new connection
    pub fn new(url: String) -> Result<RabbitMQConnection, &'static str> {
        let conn_result = Connection::insecure_open(url.as_str());
        if(conn_result.is_ok()){
            let mut conn = conn_result.unwrap();
            let channel_result = conn.open_channel(None);
            if(channel_result.is_ok()) {
                let channel = channel_result.unwrap();
                let conn_object = RabbitMQConnection {
                    connection: conn,
                    channel: channel,
                };
                Ok(conn_object)
            }else{
                match channel_result{
                    Err(e) =>{
                        println!("{}", e);
                    }
                    _ => {}
                }
                Err("Failed to Establish a Channel")
            }
        }else {
            match conn_result{
                Err(e) =>{
                    println!("{}", e);
                }
                _ => {}
            }
            Err("Failed to Establish a Connection")
        }
    }
}

