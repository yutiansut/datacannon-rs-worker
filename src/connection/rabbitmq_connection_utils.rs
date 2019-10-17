/// Handles a connection to rabbit mq.
/// Author: Andrew Evans
use amiquip::{Channel, Connection, Exchange, Publish, Result};


//Connection object
pub struct RabbitMQConnection{
    pub connection: Connection,
    pub channel: Channel,
}


///Implementation for rabbit mq connection
impl RabbitMQConnection{

    /// Close the connection. Returns whether the connection was closed
    pub fn close(self) -> bool {
        let mut result = self.channel.close();
        if(result.is_ok()){
            result = self.connection.close();
            if(result.is_ok()){
                true
            }else{
                false
            }
        }else{
            false
        }
    }
}
