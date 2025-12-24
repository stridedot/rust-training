use std::net::SocketAddr;

use anyhow::Result;
use crm_send::{
    Notification,
    config::AppConfig,
    pb::notification::{
        EmailMessage, InAppMessage, SendRequest, SmsMessage,
        notification_service_client::NotificationServiceClient,
    },
};
use futures::StreamExt as _;
use tonic::transport::{Server, server::TcpIncoming};

#[tokio::test]
async fn test_send_should_work() -> Result<()> {
    let addr = start_server().await?;

    let stream = tokio_stream::iter(vec![
        SendRequest {
            msg: Some(EmailMessage::fake().into()),
        },
        SendRequest {
            msg: Some(SmsMessage::fake().into()),
        },
        SendRequest {
            msg: Some(InAppMessage::fake().into()),
        },
    ]);

    let mut client = NotificationServiceClient::connect(format!("http://{}", addr)).await?;

    let request = tonic::Request::new(stream);

    let response = client.send(request).await?;
    let ret = response.into_inner().collect::<Vec<_>>().await;

    for item in ret {
        println!("item: {:?}", item?);
    }

    Ok(())
}

async fn start_server() -> Result<SocketAddr> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    let config = AppConfig::load()?;
    let svc = Notification::new(config).await.into_server();

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
