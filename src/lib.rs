pub use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::interval};
pub use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, thread, time::Duration};

pub async fn send_recv_loop(mut stream: TcpStream, id: u8) {
        let mut interval = interval(Duration::from_millis(1));
        let (mut read, mut write) = stream.split();

        // Message size caused lock 
        let buf = [0u8; 454862];
        let mut rbuf = [0u8; 454862];

        let mut i: usize = 0;

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    println!("{id}/{i}: Start writing data");

                    if let Err(err) = write.write_all(&buf).await {
                        println!("{id}/{i}: Error while writing data: {err}");
                        break;
                    }
                    println!("{id}/{i}: Wrote data");
                },
                result = read.read(&mut rbuf) => {
                    match result {
                        Ok(n) => println!("{id}/{i}: Read data {n} bytes"),
                        Err(err) => {
                            println!("{id}/{i}: Error while reading data: {err}");
                            break;
                        }
                    }
                }
                else => break,
            }

            i += 1;
        }
}