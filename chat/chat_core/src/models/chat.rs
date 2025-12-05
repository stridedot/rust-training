use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::ChatType;

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct Chat {
    pub id: i64,
    pub name: String,
    pub r#type: ChatType,
    pub workspace_id: i64,
    pub user_ids: Vec<i64>,
    pub created_at: DateTime<Utc>,
}
