use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct SignUpReq {
    pub username: String,
    pub email: String,
    pub password: String,
    pub workspace: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SignInReq {
    pub email: String,
    pub password: String,
}
