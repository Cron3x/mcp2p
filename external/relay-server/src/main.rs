use postcard::{to_allocvec};
use tokio::net::UdpSocket;
use std::{io};
use shared_lib::Client;

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
      
        let connect_to = client.ip;

        let server_client = Client { ip: &addr.ip().to_string(), secret: client.secret };

        let server_client_bytes = match to_allocvec(&server_client) {
            Ok(sc) => sc,
            Err(err) => panic!("TODO! Handle Error: {err}")
        };

        let len = sock.send_to(&server_client_bytes, connect_to).await?;
        println!("send: {len} bytes");
        /*let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);*/
    }
}
