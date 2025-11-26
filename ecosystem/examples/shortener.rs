use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions, prelude::FromRow};
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    Layer as _, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

const ADDR: &str = "0.0.0.0:8080";

#[derive(Deserialize)]
struct ReqShorten {
    url: String,
}

#[derive(Serialize)]
struct RespShorten {
    url: String,
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug, FromRow)]
struct UrlRecord {
    #[sqlx(default)]
    id: String,
    #[sqlx(default)]
    #[allow(dead_code)]
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = fmt::Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(console).init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Starting chat server on {}", addr);

    let db = "postgres://postgres:123456@localhost:5432/postgres";
    let state = AppState::try_new(db).await?;

    let app = Router::new()
        .route("/", routing::post(shorten))
        .route("/{:id}", routing::get(redirect))
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

impl AppState {
    async fn try_new(db: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new().max_connections(5).connect(db).await?;

        sqlx::query(
            r#"
            create table if not exists urls (
                id char(6) primary key,
                url text unique
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { db: pool })
    }

    async fn create(&self, url: &str) -> anyhow::Result<String> {
        let id = nanoid::nanoid!(6);

        let ret: UrlRecord = sqlx::query_as(
            r#"
            insert into urls (id, url)
            values ($1, $2) ON CONFLICT (url)
            DO UPDATE SET url = EXCLUDED.url  -- 假装更新（实际值不变）
            RETURNING id;
            "#,
        )
        .bind(&id)
        .bind(url)
        .fetch_one(&self.db)
        .await?;

        Ok(ret.id)
    }

    async fn get(&self, id: &str) -> anyhow::Result<String> {
        let row: UrlRecord = sqlx::query_as(
            r#"
            select url from urls where id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await?;

        Ok(row.url)
    }
}

async fn shorten(
    State(state): State<AppState>,
    Json(data): Json<ReqShorten>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = state.create(&data.url).await.map_err(|e| {
        tracing::error!("Failed to create short url: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let body = Json(RespShorten {
        url: format!("http://{ADDR}/{}", id),
    });

    Ok((StatusCode::OK, body))
}

async fn redirect(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let url = state.get(&id).await.map_err(|_| StatusCode::NOT_FOUND)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        "Location",
        url.parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok((StatusCode::FOUND, headers))
}
