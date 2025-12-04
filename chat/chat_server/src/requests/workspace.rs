use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceReq {
    pub name: String,
    pub owner_id: i64,
}
