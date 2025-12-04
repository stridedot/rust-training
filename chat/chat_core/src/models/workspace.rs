use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub created_at: DateTime<Utc>,
}
