use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct MessageListRequest {
    pub last_id: Option<i64>,
    pub limit: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct MessageSendRequest {
    pub content: String,
    #[serde(default)]
    pub files: Vec<String>,
}
