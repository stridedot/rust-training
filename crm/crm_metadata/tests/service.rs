use anyhow::Result;
use crm_metadata::{
    CrmMetadata,
    config::AppConfig,
    pb::metadata::{MaterializeRequest, metadata_service_client::MetadataServiceClient},
};
use futures::StreamExt as _;
use std::net::SocketAddr;
use tonic::transport::{Server, server::TcpIncoming};

#[tokio::test]
async fn test_materialize_should_work() -> Result<()> {
    let addr = start_server().await?;

    let stream = tokio_stream::iter(vec![
        MaterializeRequest { id: 1 },
        MaterializeRequest { id: 2 },
        MaterializeRequest { id: 3 },
    ]);

    let mut client = MetadataServiceClient::connect(format!("http://{}", addr)).await?;

    let request = tonic::Request::new(stream);
    let response = client.materialize(request).await?;
    let mut stream = response.into_inner();

    while let Some(item) = stream.next().await {
        let item = item?;
        println!("{:?}", item);
    }

    Ok(())
}

async fn start_server() -> Result<SocketAddr> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    let config = AppConfig::load()?;
    let svc = CrmMetadata::new(config).await.into_server();

    let incoming = TcpIncoming::from(listener);

    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve_with_incoming(incoming)
            .await
            .unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    Ok(addr)
}
