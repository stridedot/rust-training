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
use tonic::transport::Server;

const PORT_BASE: u32 = 6003;

#[tokio::test]
async fn test_send_should_work() -> Result<()> {
    let addr = start_server(PORT_BASE).await?;

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

async fn start_server(port: u32) -> Result<SocketAddr> {
    let addr = format!("127.0.0.1:{}", port).parse()?;

    let config = AppConfig::load()?;
    let svc = Notification::new(config).await.into_server();

    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    Ok(addr)
}
