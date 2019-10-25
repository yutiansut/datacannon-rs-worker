/*
Threadable rabbit mq connection

Author Andrew Evans
*/


use amiquip::Connection;


/// struct storage
pub struct ThreadableRabbitMQConnection{
    pub connection: Connection,
}


/// Implementation of the connection
impl ThreadableRabbitMQConnection {

    /// Create a new connection
    pub fn new(url: String) -> Result<ThreadableRabbitMQConnection, &'static str> {
        let conn_result = Connection::insecure_open(url.as_str());
        if (conn_result.is_ok()) {
            let mut conn = conn_result.unwrap();
            let channel_result = conn.open_channel(None);
            if (channel_result.is_ok()) {
                let channel = channel_result.unwrap();
                let conn_object = ThreadableRabbitMQConnection {
                    connection: conn,
                };
                Ok(conn_object)
            } else {
                match channel_result {
                    Err(e) => {
                        println!("{}", e);
                    }
                    _ => {}
                }
                Err("Failed to Establish a Channel")
            }
        } else {
            match conn_result {
                Err(e) => {
                    println!("{}", e);
                }
                _ => {}
            }
            Err("Failed to Establish a Connection")
        }
    }
}
