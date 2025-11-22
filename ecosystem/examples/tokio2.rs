use std::time::Duration;

use tokio::sync::mpsc::{self, Receiver};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel(10);
    let handler = worker(rx);

    tokio::spawn(async move {
        let mut i = 0;
        loop {
            i += 1;
            tx.send(i).await.unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    handler.await?;

    Ok(())
}

fn worker(mut rx: Receiver<i32>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("Received: {}", i);
        }
    })
}
