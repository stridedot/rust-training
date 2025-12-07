use crate::utils::ApiResponse;
use axum::{
    Json,
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // 系统错误 5xx
    #[error("DB error, {0}")]
    DBError(#[from] sqlx::Error),

    #[error("Internal server error, {0}")]
    InternalError(String),

    #[error("IO error, {0}")]
    IOError(#[from] std::io::Error),

    #[error("JWT error, {0}")]
    JwtError(#[from] jwt_simple::Error),

    // 业务错误 4xx
    #[error("Password error, {0}")]
    PasswordError(#[from] argon2::password_hash::Error),

    #[error("Forbidden, {0}")]
    Forbidden(String),

    #[error("Not found, {0}")]
    NotFound(String),

    // 客户端错误 400
    #[error("Invalid request, {0}")]
    InvalidRequest(String),

    #[error("Invalid file, {0}")]
    InvalidFile(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let status = match self {
            Self::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::JwtError(_) => StatusCode::FORBIDDEN,
            Self::PasswordError(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            Self::InvalidFile(_) => StatusCode::BAD_REQUEST,
        };

        let json: ApiResponse<()> = ApiResponse::error(status, self.to_string());
        (StatusCode::OK, Json(json)).into_response()
    }
}
