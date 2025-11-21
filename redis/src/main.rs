use redis::{backend::Backend, network};
use tokio::net::TcpListener;

/// 启动 redis 服务器：cargo run --package redis
///
/// Client：docker exec -it redis redis-cli -h 192.168.14.171 -p 6379 PING

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    println!("Server redis listens on {}", addr);

    let backend = Backend::new();

    loop {
        let backend = backend.clone();
        let (socket, addr) = listener.accept().await?;
        println!("server redis accepts connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = network::handle_stream(socket, &backend).await {
                eprintln!(
                    "Server redis Error handling connection from {}: {}",
                    addr, e
                );
            }
        });
    }
}
