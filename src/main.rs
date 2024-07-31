extern crate core_affinity;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, runtime::Builder, time::interval};
use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, thread, time::Duration};

fn main() {
    // spawn 2 threads with own runtimes
    // each spawn socket and connect to other side
    // send and receieve messages in loop { select! }
    // check that terminates

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let core_ids = core_affinity::get_core_ids().unwrap();

    let id1 = core_ids[0];
    let t1 = thread::spawn(move || {
        let res = core_affinity::set_for_current(id1);
        if res {
            spawn_thread_1(addr)
        }
    });
    let id2 = core_ids[1];
    let t2 = thread::spawn(move || {
        let res = core_affinity::set_for_current(id2);
        if res {
            spawn_thread_2(addr)
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

fn spawn_thread_1(addr: SocketAddr) {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(async {
        let listen = TcpListener::bind(addr).await.unwrap();
        let (stream, _) = listen.accept().await.unwrap();

        send_recv_loop(stream, 1).await
    });
}

fn spawn_thread_2(addr: SocketAddr) {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(async {
        let stream = TcpStream::connect(addr).await.unwrap();

        send_recv_loop(stream, 2).await
    });
}

async fn send_recv_loop(mut stream: TcpStream, id: u8) {
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