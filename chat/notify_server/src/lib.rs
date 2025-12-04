use anyhow::Result;
use axum::{Router, middleware::from_fn_with_state, response::Html, routing};
use chat_core::{
    error::AppError,
    middlewares::{TokenVerify, auth},
    models::user::User,
    utils::jwt::DecodingKey,
};
use std::{ops::Deref, sync::Arc};

use crate::{config::AppConfig, sse::sse_handler};

pub mod config;
pub mod sse;

#[derive(Clone)]
pub struct AppState(Arc<AppStateInner>);

pub struct AppStateInner {
    #[allow(dead_code)]
    config: AppConfig,
    verify_key: DecodingKey,
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self> {
        let verify_key = DecodingKey::load(&config.auth.verify_key)?;

        Ok(Self(Arc::new(AppStateInner { config, verify_key })))
    }
}

impl TokenVerify for AppState {
    type Error = AppError;

    fn verify(&self, token: &str) -> Result<User, Self::Error> {
        Ok(self.verify_key.verify(token)?)
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn get_router(state: AppState) -> Result<Router> {
    let router = Router::new()
        .route("/events", routing::get(sse_handler))
        .layer(from_fn_with_state(
            state.clone(),
            auth::verify_token::<AppState>,
        ))
        .route("/", routing::get(index_handler))
        .with_state(state);

    Ok(router)
}

async fn index_handler() -> Html<String> {
    Html(include_str!("../index.html").to_string())
}
