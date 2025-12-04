use axum::{extract::State, response::IntoResponse};

use crate::AppState;
use chat_core::error::AppError;

pub async fn file_upload(State(_): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(())
}
