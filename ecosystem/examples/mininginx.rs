use std::sync::Arc;

use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

struct Config {
    upstream_addr: String,
    listen_addr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Arc::new(resolver_config());
    let listener = TcpListener::bind(&config.listen_addr).await?;

    loop {
        let config = Arc::clone(&config);
        let (client, addr) = listener.accept().await?;
        println!("Accepted connection from: {:?}", addr);

        tokio::spawn(async move {
            let upstream = TcpStream::connect(&config.upstream_addr).await?;
            proxy(client, upstream).await?;

            Ok::<_, anyhow::Error>(())
        });
    }
}

// 启动一个有 8080 的服务端，注意必须要有个 `"/"` 的路由，如：
//    cargo run --package ecosystem --example axum_tracing
// 启动 mininginx 服务端，如：
//    cargo run --package ecosystem --example mininginx
// 客户端调用：GET http://127.0.0.1:8081
fn resolver_config() -> Config {
    Config {
        upstream_addr: "127.0.0.1:8080".to_string(),
        listen_addr: "127.0.0.1:8081".to_string(),
    }
}

async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> anyhow::Result<()> {
    let (mut client_read, mut client_write) = client.split();
    let (mut upstream_read, mut upstream_write) = upstream.split();

    let client_to_upstream = io::copy(&mut client_read, &mut upstream_write);
    let upstream_to_client = io::copy(&mut upstream_read, &mut client_write);

    match tokio::try_join!(client_to_upstream, upstream_to_client) {
        Ok((n, m)) => {
            println!("Client to upstream: {} bytes", n);
            println!("Upstream to client: {} bytes", m);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    Ok(())
}
