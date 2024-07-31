pub use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::interval};
pub use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, thread, time::Duration};

pub async fn send_recv_loop(mut stream: TcpStream, id: u8) {
        let mut interval = interval(Duration::from_millis(1));
        let (mut read, mut write) = stream.split();

        let buf = [0u8; 4096];
        let mut rbuf = [0u8; 4096];

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if let Err(err) = write.write_all(&buf).await {
                        println!("{id}: Error while writing data: {err}");
                        break;
                    }
                    println!("{id}: Wrote data");
                },
                result = read.read(&mut rbuf) => {
                    match result {
                        Ok(n) => println!("{id}: Read data {n} bytes"),
                        Err(err) => {
                            println!("{id}: Error while reading data: {err}");
                            break;
                        }
                    }
                }
                else => break,
            }
        }
}