use axum::{extract::State, response::IntoResponse};

use crate::{AppState, repos::user::UserRepo as _};
use chat_core::{
    error::AppError,
    models::user::{ChatUser, User},
    utils::ApiResponse,
};

#[utoipa::path(
    get,
    path = "/api/user/list",
    security(
        ("token" = []),
    ),
    responses(
        (status = 200, description = "User list", body = Vec<ChatUser>),
        (status = 500, description = "sqlx error"),
    ),
    tag = "user",
    description = "List all users",
)]
pub async fn user_list(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let users: Vec<ChatUser> = User::get_all(&state.pg_pool).await?;
    let resp = ApiResponse::<_>::success(users);

    Ok(resp)
}
