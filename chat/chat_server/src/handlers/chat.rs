use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    AppState,
    repos::chat::ChatRepo as _,
    requests::chat::{CreateChatReq, DeleteChatReq, UpdateChatReq},
};
use chat_core::{
    error::AppError,
    models::{chat::Chat, user::User},
    utils::ApiResponse,
};

pub async fn chat_create(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(req): Json<CreateChatReq>,
) -> Result<impl IntoResponse, AppError> {
    let chat = Chat::create(&req, user.workspace_id, &state.pg_pool).await?;
    let body = ApiResponse::success(chat);

    Ok(Json(body))
}

pub async fn chat_update(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(req): Json<UpdateChatReq>,
) -> Result<impl IntoResponse, AppError> {
    let chat = Chat::update(&req, user.id, &state.pg_pool).await?;
    let body = ApiResponse::success(chat);

    Ok(Json(body))
}

pub async fn chat_list(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, AppError> {
    let chats = Chat::get_all(user.workspace_id, &state.pg_pool).await?;
    let body = ApiResponse::success(chats);

    Ok(Json(body))
}

pub async fn chat_detail(
    State(state): State<AppState>,
    Path(chat_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let chat = Chat::find_by_id(chat_id, &state.pg_pool).await?;
    match chat {
        Some(chat) => Ok(Json(ApiResponse::success(chat))),
        None => Ok(Json(ApiResponse::error(
            StatusCode::NOT_FOUND,
            "chat not found",
        ))),
    }
}

pub async fn chat_delete(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(req): Json<DeleteChatReq>,
) -> Result<impl IntoResponse, AppError> {
    Chat::delete(req.id, user.id, &state.pg_pool).await?;

    Ok(Json(ApiResponse::success(())))
}
