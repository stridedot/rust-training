use crm_gateway::{
    config::AppConfig,
    pb::crm::{WelcomeRequest, crm_service_client::CrmServiceClient},
};
use tonic::{
    Request,
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::load()?;

    let pem = include_str!("../../fixtures/rootCA.pem");
    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(pem))
        .domain_name("localhost");

    let addr = format!("https://localhost:{}", config.server.crm_gateway_port);
    let channel = Channel::from_shared(addr.clone())?
        .tls_config(tls)?
        .connect()
        .await?;

    let token = include_str!("../../fixtures/token").trim();
    let token: MetadataValue<_> = format!("Bearer {}", token).parse()?;

    let mut client = CrmServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
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
