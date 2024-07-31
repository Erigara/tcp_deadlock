use tcp_deadlock::*;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let listen = TcpListener::bind(addr).await.unwrap();
    let (stream, _) = listen.accept().await.unwrap();
    send_recv_loop(stream, 1).await
}