use axum::{Json, extract::State, response::IntoResponse};

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
    let body = ApiResponse::success(token);

    Ok(body.into_response())
}

pub async fn sign_in(
    State(_): State<AppState>,
    Json(_): Json<SignInReq>,
) -> Result<impl IntoResponse, AppError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx_db_tester::TestPg;

    use crate::requests::user::SignUpReq;

    #[tokio::test]
    async fn test_sign_up() -> anyhow::Result<()> {
        let tdb = TestPg::new(
            "postgres://postgres:123456@localhost:5432".to_string(),
            std::path::Path::new("../migrations"),
        );

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
