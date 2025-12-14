use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateChatReq {
    pub name: Option<String>,
    pub user_ids: Vec<i64>,
    #[serde(default)]
    pub is_public: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateChatReq {
    pub id: i64,
    pub name: Option<String>,
    pub user_ids: Vec<i64>,
    #[serde(default)]
    pub is_public: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteChatReq {
    pub id: i64,
}
