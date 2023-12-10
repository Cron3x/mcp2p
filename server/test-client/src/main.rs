#[derive(Debug, Serialize, Deserialize)]
struct Client<'a> {
    secret: &'a str,
    connect_to: &'a str,
}

use serde::{Serialize, Deserialize};
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8088").await?;
    
    let client: Vec<u8>  = postcard::to_allocvec(&Client{
        secret: "whooSpooky",
        connect_to: "127.0.0.1:25565",
    }).unwrap();
    
    sock.connect("127.0.0.1:8080").await?;

    let len = sock.send(&client).await?;
    println!("{:?} bytes sent", len);
    /*loop {

    }*/

    Ok(())
}
