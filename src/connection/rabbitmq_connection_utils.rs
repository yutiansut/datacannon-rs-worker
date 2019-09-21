/// Handles a connection to rabbit mq.
/// Author: Andrew Evans
use amiquip::{Channel, Connection, Exchange, Publish, Result};


//Connection object
pub struct RabbitMQConnection{
    connection: Connection,
    channel: Channel,
}


///Implementation for rabbit mq connection
impl RabbitMQConnection{

    /// Close the connection. Returns whether the connection was closed
    fn close(self) -> bool {
        let mut result = self.channel.close();
        if(result.is_ok()){
            result = self.connection.close()
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


#[cfg(test)]
mod tests {

    #[test]
    fn should_create_connection(){

    }

    #[test]
    fn should_publish_message(){

    }

    #[test]
    fn should_bind_and_wait_for_message(){

    }

    #[test]
    fn should_close_connection(){

    }
}