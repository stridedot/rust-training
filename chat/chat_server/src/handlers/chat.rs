use axum::{extract::State, response::IntoResponse};

use crate::AppState;
use chat_core::error::AppError;

pub async fn chat_list(State(_): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(())
}

pub async fn chat_create(State(_): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(())
}

pub async fn chat_detail(State(_): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(())
}

pub async fn chat_update(State(_): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(())
}

pub async fn chat_delete(State(_): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(())
}
