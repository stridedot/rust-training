use axum::{extract::State, response::IntoResponse};

use crate::{AppState, repos::user::UserRepo as _};
use chat_core::{
    error::AppError,
    models::user::{ChatUser, User},
    utils::ApiResponse,
};

pub async fn user_list(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let users: Vec<ChatUser> = User::get_all(&state.pg_pool).await?;
    let resp = ApiResponse::<_>::success(users);

    Ok(resp.into_response())
}
