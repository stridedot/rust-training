use std::{io, net::SocketAddr};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const BUF_SIZE: usize = 1024;

// 运行: cargo run --package conc --example dredis
// 测试: docker exec -it redis redis-cli -h 192.168.14.171 -p 6379 PING
// 或：  docker exec -it redis redis-cli -h 172.19.240.1 -p 6379 PING
// docker exec -it redis redis-cli -h 192.168.14.171 -p 6379 进入 redis-cli 交互模式

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    println!("listening on {}", addr);

    loop {
        let (mut socket, raddr) = listener.accept().await?;
        println!("accepted connection from {:?}", socket.peer_addr()?);

        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(&mut socket, raddr).await {
                println!("error processing connection: {:?}", e);
            }
        });
    }
}

async fn process_redis_conn(socket: &mut TcpStream, raddr: SocketAddr) -> anyhow::Result<()> {
    loop {
        socket.readable().await?;
        let mut buf = vec![0u8; BUF_SIZE];

        match socket.read(&mut buf).await {
            // socket closed
            Ok(0) => break,
            Ok(n) => {
                // client: redis-cli -h 192.168.14.171 -p 6379 llen mylist
                // server: *2\r\n$4\r\nllen\r\n$6\r\nmylist\r\n
                let line = String::from_utf8_lossy(&buf[0..n]);
                println!("{:?}", line);

                socket.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(ref e) if e.kind() == io::ErrorKind::ConnectionReset => {
                println!("client closed connection {:?}", raddr);
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
        };

        // Write the data back
        socket.write_all(b"+PONG\r\n").await?;
    }

    println!("connection closed {:?}", raddr);
    Ok(())
}
