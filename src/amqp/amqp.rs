/*
AMQP Utilities

Author Andrew Evans
*/


/// Struct for connection information
#[derive(Clone, Debug)]
pub struct AMQPConnectionInf{
    protocol: String,
    host: String,
    port: i64,
    vhost: Option<String>,
    username: Option<String>,
    password: Option<String>,
    is_ssl: bool,
}


/// Implementation of the connection information
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

    pub fn is_ssl(&self) -> bool{
        self.is_ssl.clone()
    }

    pub fn to_url(&self) -> String {
        let cinf = self.clone();
        let mut url = "".to_string();
        if cinf.username.is_some() && cinf.password.is_some(){
            url = format!("{}://{}:{}@{}:{}", cinf.protocol, cinf.username.unwrap(), cinf.password.unwrap(), cinf.host, cinf.port);
        }else{
            url = format!("{}://{}:{}", cinf.protocol, cinf.host, cinf.port);
        }
        if self.vhost.is_some(){
            url = format!("{}/{}", url, cinf.vhost.unwrap());
        }
        url
    }

    pub fn new(protocol: String, host: String, port: i64, vhost: Option<String>, username: Option<String>, password: Option<String>, is_ssl: bool) -> AMQPConnectionInf{
        AMQPConnectionInf{
            protocol: protocol,
            host: host,
            port: port,
            vhost: vhost,
            username: username,
            password: password,
            is_ssl: is_ssl,
        }
    }
}


#[cfg(test)]
mod tests{
    use crate::amqp::amqp::AMQPConnectionInf;

    #[test]
    pub fn test_create_url(){
        let cinf = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 3030, None, None, None, false);
        let url = cinf.to_url();
        assert!(url.eq("amqp://127.0.0.1:3030"));
    }
}