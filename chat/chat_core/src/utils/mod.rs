use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod jwt;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub code: u16,
    pub error: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            code: 200,
            error: None,
            data: Some(data),
        }
    }

    pub fn error(code: impl Into<u16>, error: impl Into<String>) -> Self {
        Self {
            success: false,
            code: code.into(),
            error: Some(error.into()),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, axum::Json(self)).into_response()
    }
}
