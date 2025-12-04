use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageListRequest {
    pub last_id: Option<isize>,
    pub limit: Option<isize>,
}

#[derive(Debug, Deserialize)]
pub struct MessageSendRequest {
    pub content: String,
    #[serde(default)]
    pub files: Vec<String>,
}
