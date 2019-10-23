/*
AMQP Utilities

Author Andrew Evans
*/


pub struct AQMPConnectionInf{
    protocol: String,
    host: String,
    port: i64,
    vhost: Option<String>,
    username: Option<String>,
    password: Option<String>,
    is_ssl: bool,
}
