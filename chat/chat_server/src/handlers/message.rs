use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    requests::message::{MessageListRequest, MessageSendRequest},
};
use chat_core::error::AppError;

use axum::extract::Path;

pub async fn message_list(
    State(_): State<AppState>,
    Path(_): Path<isize>,
    Query(_): Query<MessageListRequest>,
) -> Result<impl IntoResponse, AppError> {
    Ok(())
}

pub async fn message_send(
    State(_): State<AppState>,
    Path(_): Path<isize>,
    Json(_): Json<MessageSendRequest>,
) -> Result<impl IntoResponse, AppError> {
    Ok(())
}
