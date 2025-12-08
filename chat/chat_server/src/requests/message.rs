use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageListRequest {
    pub last_id: Option<i64>,
    pub limit: Option<isize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageSendRequest {
    pub content: String,
    #[serde(default)]
    pub files: Vec<String>,
}
