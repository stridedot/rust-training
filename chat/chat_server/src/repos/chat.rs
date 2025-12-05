use chat_core::{
    error::AppError,
    models::{ChatType, chat::Chat},
};
use sqlx::PgPool;

use crate::requests::chat::CreateChatReq;

pub trait ChatRepo: Sized {
    fn create(
        req: &CreateChatReq,
        pg: &PgPool,
    ) -> impl std::future::Future<Output = Result<Self, AppError>> + Send;

    fn find_by_id(
        id: i64,
        pg: &PgPool,
    ) -> impl std::future::Future<Output = Result<Self, AppError>> + Send;

    fn get_all(
        workspace_id: i64,
        pg: &PgPool,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, AppError>> + Send;

    fn delete(
        id: i64,
        pg: &PgPool,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    fn is_chat_member(
        chat_id: i64,
        user_id: i64,
        pg: &PgPool,
    ) -> impl std::future::Future<Output = Result<bool, AppError>> + Send;
}

impl ChatRepo for Chat {
    async fn create(req: &CreateChatReq, pg: &PgPool) -> Result<Self, AppError> {
        let len = req.user_ids.len();
        let chat_type = match (req.is_public, len) {
            (_, 2) => ChatType::Single,
            (_, 3..=10) => ChatType::Group,
            (false, _) => ChatType::PrivateChannel,
            (true, _) => ChatType::PublicChannel,
        };

        let chat = sqlx::query_as(
            r#"
            insert into chat (name, r#type, workspace_id, user_ids)
            values ($1, $2, $3, $4)
            returning id, name, r#type, workspace_id, user_ids, created_at
            "#,
        )
        .bind(&req.name)
        .bind(chat_type)
        .bind(req.workspace_id)
        .fetch_one(pg)
        .await?;

        Ok(chat)
    }

    async fn find_by_id(id: i64, pg: &PgPool) -> Result<Self, AppError> {
        let chat = sqlx::query_as(
            r#"
            select id, name, workspace_id, created_at
            from chats
            where id = $1
            "#,
        )
        .bind(id)
        .fetch_one(pg)
        .await?;

        Ok(chat)
    }

    async fn get_all(workspace_id: i64, pg: &PgPool) -> Result<Vec<Self>, AppError> {
        let chats = sqlx::query_as(
            r#"
            select id, name, workspace_id, created_at
            from chats
            where workspace_id = $1
            "#,
        )
        .bind(workspace_id)
        .fetch_all(pg)
        .await?;

        Ok(chats)
    }

    async fn delete(id: i64, pg: &PgPool) -> Result<(), AppError> {
        sqlx::query(
            r#"
            delete from chats
            where id = $1
            "#,
        )
        .bind(id)
        .execute(pg)
        .await?;

        Ok(())
    }

    async fn is_chat_member(chat_id: i64, user_id: i64, pg: &PgPool) -> Result<bool, AppError> {
        let is_member = sqlx::query(
            r#"
            select 1
            from chat
            where id = $1
            and $2 = any(user_ids)
            "#,
        )
        .bind(chat_id)
        .bind(user_id)
        .fetch_optional(pg)
        .await?;

        Ok(is_member.is_some())
    }
}
