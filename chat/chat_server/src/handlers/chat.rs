use axum::{Extension, Json, extract::State, response::IntoResponse};

use crate::{AppState, requests::chat::CreateChatReq};
use chat_core::{error::AppError, models::user::User};

pub async fn chat_list(State(_): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(())
}

pub async fn chat_create(
    State(_): State<AppState>,
    Extension(_): Extension<User>,
    Json(_): Json<CreateChatReq>,
) -> Result<impl IntoResponse, AppError> {
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
