use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateChatReq {
    pub name: Option<String>,
    pub workspace_id: i64,
    pub user_ids: Vec<i64>,
    #[serde(default)]
    pub is_public: bool,
}
