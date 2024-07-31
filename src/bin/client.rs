use tcp_deadlock::*;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let stream = TcpStream::connect(addr).await.unwrap();
    send_recv_loop(stream, 2).await
}