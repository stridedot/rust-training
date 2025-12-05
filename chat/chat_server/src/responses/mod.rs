use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthOutput {
    pub token: String,
}
