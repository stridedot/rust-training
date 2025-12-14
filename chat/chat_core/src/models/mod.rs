use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod chat;
pub mod message;
pub mod user;
pub mod workspace;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "chat_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Single,
    Group,
    PrivateChannel,
    PublicChannel,
}
