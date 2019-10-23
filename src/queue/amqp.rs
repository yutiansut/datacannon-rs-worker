/*
AMQP Utilities

Author Andrew Evans
*/


#[derive(Clone)]
pub struct AMQPConnectionInf{
    pub protocol: String,
    pub host: String,
    pub port: i64,
    pub vhost: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_ssl: bool,
}

impl AMQPConnectionInf{

    pub fn get_protocol(&self) -> String{
        self.protocol.clone()
    }

    pub fn get_host(&self) -> String{
        self.host.clone()
    }

    pub fn get_port(&self) -> i64{
        self.port.clone()
    }

    pub fn get_vhost(&self) -> Option<String>{
        self.vhost.clone()
    }

    pub fn get_username(&self) -> Option<String>{
        self.username.clone()
    }

    pub fn get_password(&self) -> Option<String>{
        self.password.clone()
    }

    pub fn get_ssl(&self) -> bool{
        self.is_ssl.clone()
    }
}
