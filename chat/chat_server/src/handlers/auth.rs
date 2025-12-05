use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::responses::AuthOutput;
use crate::{
    AppState,
    repos::user::UserRepo,
    requests::user::{SignInReq, SignUpReq},
};
use chat_core::{error::AppError, models::user::User, utils::ApiResponse};

pub async fn sign_up(
    State(state): State<AppState>,
    Json(req): Json<SignUpReq>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::create(&req, &state.pg_pool).await?;
    let token = state.sign_key.sign(user)?;

    Ok(ApiResponse::success(AuthOutput { token }))
}

pub async fn sign_in(
    State(state): State<AppState>,
    Json(req): Json<SignInReq>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::find_by_email(&req.email, &state.pg_pool).await?;
    let body = match user {
        Some(user) => {
            let token = state.sign_key.sign(user)?;
            ApiResponse::success(AuthOutput { token })
        }
        None => ApiResponse::error(
            StatusCode::FORBIDDEN,
            format!("user not found: {}", req.email),
        ),
    };

    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requests::user::SignUpReq;

    #[tokio::test]
    async fn test_sign_up() -> anyhow::Result<()> {
        let (tdb, _) = AppState::test_new().await?;

        let pool = tdb.get_pool().await;

        let req = SignUpReq {
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "test".to_string(),
            workspace: "test".to_string(),
        };
        let user = User::create(&req, &pool).await?;
        assert_eq!(user.username, req.username);
        assert_eq!(user.email, req.email);

        Ok(())
    }
}
