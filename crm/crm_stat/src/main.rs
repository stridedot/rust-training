use crm_stat::{UserStat, config::AppConfig};
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
    let port = config.server.user_stat_port;

    let user_stat = UserStat::try_new(config).await?;
    let svc = user_stat.into_server();

    let addr = format!("[::1]:{}", port).parse()?;

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
