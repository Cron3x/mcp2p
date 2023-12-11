use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Client<'a>{
    pub ip: &'a str,
    pub secret: &'a str,
}
impl<'a> Client<'a> {
    pub fn new(ip:&'a str, secret:&'a str) -> Self{
        Self{
            ip: &ip,
            secret: &secret,
        }
    }
}
