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

#[utoipa::path(
    post,
    path = "/api/chat/create",
    security(
        ("token" = []),
    ),
    request_body = CreateChatReq,
    responses(
        (status = 200, description = "Chat created successfully", body = Chat),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "sqlx error"),
    ),
    tag = "chat",
    description = "Create a new chat",
)]
pub async fn chat_create(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(req): Json<CreateChatReq>,
) -> Result<impl IntoResponse, AppError> {
    let chat = Chat::create(&req, user.workspace_id, &state.pg_pool).await?;
    let body = ApiResponse::success(chat);

    Ok(Json(body))
}

#[utoipa::path(
    put,
    path = "/api/chat/update",
    security(
        ("token" = []),
    ),
    request_body = UpdateChatReq,
    responses(
        (status = 200, description = "Chat updated successfully", body = Chat),
        (status = 400, description = "Invalid request"),
        (status = 403, description = "User is not chat member"),
        (status = 404, description = "Chat not found"),
        (status = 500, description = "sqlx error"),
    ),
    tag = "chat",
    description = "Update a chat",
)]
pub async fn chat_update(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(req): Json<UpdateChatReq>,
) -> Result<impl IntoResponse, AppError> {
    let chat = Chat::update(&req, user.id, &state.pg_pool).await?;
    let body = ApiResponse::success(chat);

    Ok(Json(body))
}

#[utoipa::path(
    get,
    path = "/api/chat/list",
    responses(
        (status = 200, description = "Chat list", body = Vec<Chat>),
        (status = 500, description = "sqlx error"),
    ),
    tag = "chat",
    description = "List all chats",
    security(
        ("token" = [])
    )
)]
pub async fn chat_list(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, AppError> {
    let chats = Chat::get_all(user.workspace_id, &state.pg_pool).await?;
    let body = ApiResponse::success(chats);

    Ok(Json(body))
}

#[utoipa::path(
    get,
    path = "/api/chat/{chat_id}",
    security(
        ("token" = []),
    ),
    params(
        ("chat_id" = i64, Path, description = "Chat ID"),
    ),
    responses(
        (status = 200, description = "Chat detail found successfully", body = Chat),
        (status = 404, description = "Chat not found"),
        (status = 500, description = "sqlx error"),
    ),
    tag = "chat",
    description = "Get chat detail",
)]
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

#[utoipa::path(
    delete,
    path = "/api/chat/delete",
    security(
        ("token" = []),
    ),
    request_body = DeleteChatReq,
    responses(
        (status = 200, description = "Chat deleted successfully"),
        (status = 404, description = "Chat not found"),
        (status = 500, description = "sqlx error"),
    ),
    tag = "chat",
    description = "Delete a chat",
)]
pub async fn chat_delete(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(req): Json<DeleteChatReq>,
) -> Result<impl IntoResponse, AppError> {
    Chat::delete(req.id, user.id, &state.pg_pool).await?;

    Ok(Json(ApiResponse::success(())))
}
