use serde::{Serialize, Deserialize};
use tokio::net::UdpSocket;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct ServerClient<'a> {
    ip: &'a str,
    client: Client<'a>
}
#[derive(Debug, Serialize, Deserialize)]
struct Client<'a> {
    secret: &'a str,
    connect_to: &'a str,
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        println!("--- Message ---");
        
        let client: Client = postcard::from_bytes(&buf[..len]).unwrap();

        println!("{client:#?}");
        println!("--- End ---");
        

        /*let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);*/
    }
}
