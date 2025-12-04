use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignUpReq {
    pub username: String,
    pub email: String,
    pub password: String,
    pub workspace: String,
}

#[derive(Debug, Deserialize)]
pub struct SignInReq {
    pub email: String,
    pub password: String,
}
