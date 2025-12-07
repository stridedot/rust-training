use axum::{Router, middleware::from_fn_with_state, routing};
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tokio::fs;

use crate::{
    config::AppConfig,
    handlers::{
        auth::{sign_in, sign_up},
        chat::{chat_create, chat_delete, chat_detail, chat_list, chat_update},
        file::{file_download, file_upload},
        message::{message_list, message_send},
        user::user_list,
    },
};

use chat_core::{
    error::AppError,
    middlewares::{TokenVerify, auth},
    models::user::User,
    utils::jwt::{DecodingKey, EncodingKey},
};

pub mod config;
pub mod handlers;
pub mod middlewares;
pub mod repos;
pub mod requests;
pub mod responses;

#[derive(Clone, Debug)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[derive(Clone, Debug)]
pub struct AppStateInner {
    pub config: AppConfig,
    pg_pool: PgPool,
    sign_key: EncodingKey,
    verify_key: DecodingKey,
}

impl Deref for AppState {
    type Target = Arc<AppStateInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        let pg_pool = PgPool::connect(&config.server.pg_url)
            .await
            .map_err(AppError::DBError)?;

        fs::create_dir_all(&config.server.file_dir)
            .await
            .map_err(AppError::IOError)?;

        let sign_key = EncodingKey::load(&config.auth.sign_key)?;
        let verify_key = DecodingKey::load(&config.auth.verify_key)?;

        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                pg_pool,
                sign_key,
                verify_key,
            }),
        })
    }
}

impl TokenVerify for AppState {
    type Error = AppError;

    fn verify(&self, token: &str) -> Result<User, Self::Error> {
        Ok(self.verify_key.verify(token)?)
    }
}

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    // 定义 message 路由
    let message = Router::new()
        .route("/{:chat_id}/message/list", routing::get(message_list))
        .route("/{:chat_id}/message/send", routing::post(message_send));

    let api = Router::new()
        .route("/user/list", routing::get(user_list))
        .route("/chat/list", routing::get(chat_list))
        .route("/chat/create", routing::post(chat_create))
        .route("/chat/update", routing::post(chat_update))
        .route("/chat/delete", routing::post(chat_delete))
        .route("/chat/{:chat_id}", routing::get(chat_detail))
        .route("/upload", routing::post(file_upload))
        .nest("/chat", message)
        .route(
            "/download/{:workspace_id}/{*path}",
            routing::get(file_download),
        )
        .layer(from_fn_with_state(
            state.clone(),
            auth::verify_token::<AppState>,
        ))
        .route("/sign-up", routing::post(sign_up))
        .route("/sign-in", routing::post(sign_in));

    let app = Router::new()
        .route("/", routing::get(|| async { "Hello, World!" }))
        .nest("/api", api)
        .with_state(state);

    Ok(app)
}

#[cfg(feature = "test-util")]
pub mod test_util {
    use super::*;
    use anyhow::Context as _;
    use sqlx::{Executor, PgPool};

    impl AppState {
        pub async fn test_new() -> Result<(sqlx_db_tester::TestPg, Self), AppError> {
            let config = AppConfig::load()?;
            let _ = tokio::fs::create_dir_all(&config.server.file_dir)
                .await
                .context("create file_dir failed");

            let sign_key =
                EncodingKey::load(&config.auth.sign_key).context("load sign key failed")?;
            let verify_key =
                DecodingKey::load(&config.auth.verify_key).context("load verify key failed")?;

            let post = config.server.pg_url.rfind('/').expect("invalid pg_url");
            let server_url = &config.server.pg_url[..post];
            let (tdb, pool) = Self::get_test_pool(server_url).await;

            let state = Self {
                inner: Arc::new(AppStateInner {
                    config,
                    pg_pool: pool,
                    sign_key,
                    verify_key,
                }),
            };
            Ok((tdb, state))
        }

        pub async fn get_test_pool(server_url: &str) -> (sqlx_db_tester::TestPg, PgPool) {
            let tdb = sqlx_db_tester::TestPg::new(
                server_url.to_string(),
                std::path::Path::new("../migrations"),
            );
            println!("Using pg_url: {}, dbname: {}", tdb.server_url, tdb.dbname);
            let pool = tdb.get_pool().await;

            // run prepared sql to insert test data
            let sql = include_str!("../../chat_core/fixtures/test.sql").split(';');
            let mut ts = pool.begin().await.expect("begin transaction failed");
            for s in sql {
                if s.trim().is_empty() {
                    continue;
                }
                ts.execute(s).await.expect("execute sql failed");
            }
            ts.commit().await.expect("commit transaction failed");

            (tdb, pool)
        }
    }
}
