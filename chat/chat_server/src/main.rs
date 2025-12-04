use std::sync::OnceLock;

use anyhow::Result;
use chat_server::{AppState, config::AppConfig, get_router};
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling;
use tracing_subscriber::{
    Layer,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt as _,
    util::SubscriberInitExt as _,
};

static LOG_GUARD: OnceLock<tracing_appender::non_blocking::WorkerGuard> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    if let Err(e) = run().await {
        tracing::error!("{}", e);
        return Err(e);
    }

    Ok(())
}

fn init_tracing() {
    // 输出到控制台
    let console_layer = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_filter(LevelFilter::INFO);

    // 输出到日志
    let log_dir: String = format!("logs/{}", env!("CARGO_PKG_NAME"));
    let file_appender = rolling::daily(&log_dir, "chat_server.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // 把 guard 保存起来，防止 drop
    LOG_GUARD.set(guard).ok();

    let file_layer = fmt::Layer::new()
        .with_ansi(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();
}

async fn run() -> Result<()> {
    let config = AppConfig::load()?;

    let state = AppState::try_new(config.clone()).await?;
    let app = get_router(state).await?;

    let addr = format!("{}:{}", config.server.host, config.server.http_port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Chat server is running on: {}", addr);

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
