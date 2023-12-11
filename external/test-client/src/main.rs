use tokio::net::UdpSocket;
use std::{io, env};
use shared_lib::Client;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8088").await?;

    let mode: String = env::args().nth(1).unwrap();
    if mode == "connect" { println!("try to connect") }
    let scnd = env::args().nth(2).unwrap(); 
    let (ip, secret) = scnd.split_once("+").unwrap();

    let client: Vec<u8> = postcard::to_allocvec( &Client{
        ip,
        secret
    } ).unwrap();
    
    let mut buf = [0; 1024]; 
    sock.connect("127.0.0.1:8080").await?;

    let len = sock.send(&client).await?;
    println!("{:?} bytes sent", len);
    let (len, addr) = sock.recv_from(&mut buf).await?;
    println!("receved {len} bytes from {addr}");

    Ok(())
}
