use crm_gateway::{
    config::AppConfig,
    pb::crm::{WelcomeRequest, crm_service_client::CrmServiceClient},
};
use tonic::Request;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::load()?;

    let addr = format!("http://localhost:{}", config.server.crm_gateway_port);
    let mut client = CrmServiceClient::connect(addr.clone()).await?;
    println!("addr: {}", addr);

    let req = Request::new(WelcomeRequest {
        id: Uuid::new_v4().to_string(),
        interval: 93_u32,
        content_ids: vec![1_u32, 2, 3],
    });

    let resp = client.welcome(req).await?.into_inner();
    println!("response = {:?}", resp);

    Ok(())
}
