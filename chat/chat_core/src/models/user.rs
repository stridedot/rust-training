use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, FromRow, Serialize, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[sqlx(default)]
    #[serde(skip)]
    pub password: String,
    #[sqlx(default)]
    pub workspace_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize, ToSchema)]
pub struct ChatUser {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub workspace_id: i64,
    pub created_at: DateTime<Utc>,
}
