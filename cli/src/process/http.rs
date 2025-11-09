use std::{path::PathBuf, sync::Arc};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};
use tower_http::services::ServeDir;

pub struct HttpServeState {
    dir: PathBuf,
}

pub async fn process_http_serve(dir: &PathBuf, port: u16) -> anyhow::Result<()> {
    let state = HttpServeState { dir: dir.into() };
    let service = ServeDir::new(state.dir.clone())
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();

    let router = Router::new()
        // 只匹配 /cli/fixtures 开头的路径
        .nest_service("/cli/fixtures", service)
        // 匹配所有其他路径
        .route("/{*path}", routing::get(file_handler))
        .with_state(Arc::new(state));

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, router).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let path = state.dir.join(path);
    if !path.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {:?} not found", path.display()),
        )
    } else {
        let content = tokio::fs::read_to_string(path).await.unwrap();
        (StatusCode::OK, content)
    }
}
