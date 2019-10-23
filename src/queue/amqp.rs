/*
AMQP Utilities

Author Andrew Evans
*/


pub struct AMQPConnectionInf{
    protocol: String,
    host: String,
    port: i64,
    vhost: Option<String>,
    username: Option<String>,
    password: Option<String>,
    is_ssl: bool,
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
