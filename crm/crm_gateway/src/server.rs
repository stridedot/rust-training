use crm_gateway::{CrmGateway, config::AppConfig};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::load()?;
    let addr = format!("{}:{}", config.server.host, config.server.crm_gateway_port);

    let svc = CrmGateway::try_new(config).await?.into_server();
    Server::builder()
        .add_service(svc)
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
