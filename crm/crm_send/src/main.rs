use crm_send::{Notification, config::AppConfig};
use tonic::transport::Server;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    Layer as _, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = fmt::Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(console).init();

    let config = AppConfig::load()?;
    let port = config.server.notify_port;

    let notification = Notification::new(config).await;
    let svc = notification.into_server();

    let addr = format!("[::1]:{}", port).parse()?;
    tracing::info!("notification server is running on: {}", addr);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
