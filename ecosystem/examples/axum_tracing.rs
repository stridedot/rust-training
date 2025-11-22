use std::{sync::OnceLock, time::Duration};

use axum::{Router, routing};
use tokio::{join, net::TcpListener, time::Instant};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_appender::rolling;
use tracing_subscriber::{
    Layer,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt as _,
    util::SubscriberInitExt as _,
};

use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_otlp::{Protocol, SpanExporter};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 输出到控制台
    let console_layer = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_filter(LevelFilter::INFO);

    // 输出到日志
    let file_appender = rolling::daily(
        format!("logs/{}", env!("CARGO_PKG_NAME")),
        "axum_tracing.log",
    );
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .with_filter(LevelFilter::INFO);

    let tracer_provider = init_traces().await;
    global::set_tracer_provider(tracer_provider.clone());

    let tracer = tracer_provider.tracer("basic");
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .with(otel_layer)
        .init();

    let addr = "0.0.0.0:8080";
    let app = Router::new().route("/", routing::get(index_handler));
    let listener = TcpListener::bind(addr).await?;

    info!("Starting server on {}", addr);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[tracing::instrument(fields(http.path = req.uri().path(), http.method = req.method().as_str()))]
async fn index_handler(req: axum::extract::Request) -> &'static str {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    let ret = long_task().await;
    info!(http.status_code = 200, "index handler completed");

    ret
}

#[tracing::instrument]
async fn long_task() -> &'static str {
    let start = Instant::now();
    let sl = tokio::time::sleep(Duration::from_millis(50));

    let t1 = task1();
    let t2 = task2();
    let t3 = task3();

    join!(sl, t1, t2, t3);

    let elapsed = start.elapsed().as_millis() as u64;
    warn!(app.task_duration = elapsed, "task takes too long");

    "Hello, World!"
}

#[tracing::instrument]
async fn task1() {
    tokio::time::sleep(Duration::from_millis(100)).await;
}

#[tracing::instrument]
async fn task2() {
    tokio::time::sleep(Duration::from_millis(200)).await;
}

#[tracing::instrument]
async fn task3() {
    tokio::time::sleep(Duration::from_millis(300)).await;
}

async fn init_traces() -> SdkTracerProvider {
    // docker run -d --name jaeger -p16686:16686 -p4317:4317 -p4318:4318 jaegertracing/all-in-one:latest
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_protocol(Protocol::Grpc)
        .with_endpoint("http://127.0.0.1:4317")
        .build()
        .expect("Failed to create trace exporter");

    SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource())
        .build()
}

fn get_resource() -> Resource {
    static RESOURCE: OnceLock<Resource> = OnceLock::new();
    RESOURCE
        .get_or_init(|| {
            Resource::builder()
                .with_service_name(format!("rust-training-{}", env!("CARGO_PKG_NAME")))
                .build()
        })
        .clone()
}
