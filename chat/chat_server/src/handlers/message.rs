use std::str::FromStr as _;

use axum::{
    Extension, Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::repos::message::MessageRepo as _;
use crate::{
    AppState,
    repos::{chat::ChatRepo as _, file::ChatFile},
    requests::message::{MessageListRequest, MessageSendRequest},
};
use chat_core::{
    error::AppError,
    models::{chat::Chat, message::Message, user::User},
    utils::ApiResponse,
};

use axum::extract::Path;

#[utoipa::path(
    get,
    path = "/api/chat/{chat_id}/message/list",
    security(
        ("token" = []),
    ),
    params(
        ("chat_id" = i64, Path, description = "Chat ID"),
    ),
    responses(
        (status = 200, description = "Message list found successfully", body = Vec<Message>),
        (status = 403, description = "User is not chat member"),
        (status = 500, description = "sqlx error"),
    ),
    tag = "chat",
)]
pub async fn message_list(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(chat_id): Path<i64>,
    Query(req): Query<MessageListRequest>,
) -> Result<impl IntoResponse, AppError> {
    if !Chat::is_chat_member(chat_id, user.id, &state.pg_pool).await? {
        return Err(AppError::Forbidden(format!(
            "user {} is not chat member",
            user.id
        )));
    }

    let msgs = Message::get_all(chat_id, &req, &state.pg_pool).await?;

    Ok(ApiResponse::success(msgs))
}

#[utoipa::path(
    post,
    path = "/api/chat/{chat_id}/message/send",
    security(
        ("token" = []),
    ),
    params(
        ("chat_id" = i64, Path, description = "Chat ID"),
    ),
    responses(
        (status = 200, description = "Message sent successfully", body = Message),
        (status = 400, description = "Invalid request"),
        (status = 403, description = "User is not chat member"),
        (status = 404, description = "File not found"),
        (status = 500, description = "sqlx error"),
    ),
    tag = "chat",
)]
pub async fn message_send(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(chat_id): Path<i64>,
    Json(req): Json<MessageSendRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1、检查请求参数
    if req.content.is_empty() && req.files.is_empty() {
        return Err(AppError::InvalidRequest(
            "no content or file uploaded".to_string(),
        ));
    }

    // 2、检查权限
    if !Chat::is_chat_member(chat_id, user.id, &state.pg_pool).await? {
        return Err(AppError::Forbidden(format!(
            "user {} is not chat member",
            user.id
        )));
    }

    let file_dir = &state.config.server.file_dir;

    // 3、检查文件是否存在
    for file_path in &req.files {
        let chat_file = ChatFile::from_str(file_path)?;
        if !chat_file.path(file_dir).exists() {
            return Err(AppError::NotFound(format!("file {} not exists", file_path)));
        }
    }

    let message = Message::create(chat_id, user.id, req, &state.pg_pool).await?;
    let body = ApiResponse::success(message);

    Ok(Json(body))
}
