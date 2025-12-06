use chat_core::{
    error::AppError,
    models::{ChatType, chat::Chat, user::User},
};
use sqlx::PgPool;

use crate::{
    repos::user::UserRepo,
    requests::chat::{CreateChatReq, UpdateChatReq},
};

pub trait ChatRepo: Sized {
    fn create(
        req: &CreateChatReq,
        workspace_id: i64,
        pg: &PgPool,
    ) -> impl std::future::Future<Output = Result<Self, AppError>> + Send;

    fn update(
        req: &UpdateChatReq,
        pg: &PgPool,
    ) -> impl std::future::Future<Output = Result<Self, AppError>> + Send;

    fn find_by_id(
        id: i64,
        pg: &PgPool,
    ) -> impl std::future::Future<Output = Result<Option<Self>, AppError>> + Send;

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
    async fn create(req: &CreateChatReq, workspace_id: i64, pg: &PgPool) -> Result<Self, AppError> {
        if req.user_ids.is_empty() {
            return Err(AppError::InvalidRequest("user_ids is empty".to_string()));
        }
        let len = req.user_ids.len();

        let user_ids = User::find_by_ids(&req.user_ids, pg).await?;
        if user_ids.len() != len {
            return Err(AppError::InvalidRequest("user not exists".to_string()));
        }

        let chat_type = match (req.is_public, len) {
            (_, 2) => ChatType::Single,
            (_, 3..=10) => ChatType::Group,
            (false, _) => ChatType::PrivateChannel,
            (true, _) => ChatType::PublicChannel,
        };

        let chat = sqlx::query_as(
            r#"
            insert into chat (name, type, workspace_id, user_ids)
            values ($1, $2, $3, $4)
            returning id, name, type, workspace_id, user_ids, created_at
            "#,
        )
        .bind(&req.name)
        .bind(chat_type)
        .bind(workspace_id)
        .bind(&req.user_ids)
        .fetch_one(pg)
        .await?;

        Ok(chat)
    }

    async fn update(req: &UpdateChatReq, pg: &PgPool) -> Result<Self, AppError> {
        let chat = Self::find_by_id(req.id, pg).await?;
        if chat.is_none() {
            return Err(AppError::NotFound("chat not exists".to_string()));
        }

        if req.user_ids.is_empty() {
            return Err(AppError::InvalidRequest("user_ids is empty".to_string()));
        }
        let len = req.user_ids.len();

        let user_ids = User::find_by_ids(&req.user_ids, pg).await?;
        if user_ids.len() != len {
            return Err(AppError::InvalidRequest("user not exists".to_string()));
        }

        let chat_type = match (req.is_public, len) {
            (_, 2) => ChatType::Single,
            (_, 3..=10) => ChatType::Group,
            (false, _) => ChatType::PrivateChannel,
            (true, _) => ChatType::PublicChannel,
        };

        let chat = sqlx::query_as(
            r#"
            update chat
            set
                name = $1,
                type = $2,
                user_ids = $3
            where id = $4
            returning id, name, type, workspace_id, user_ids, created_at
            "#,
        )
        .bind(&req.name)
        .bind(chat_type)
        .bind(&req.user_ids)
        .fetch_one(pg)
        .await?;

        Ok(chat)
    }

    async fn find_by_id(id: i64, pg: &PgPool) -> Result<Option<Self>, AppError> {
        let chat = sqlx::query_as(
            r#"
            select id, name, type, workspace_id, user_ids, created_at
            from chat
            where id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pg)
        .await?;

        Ok(chat)
    }

    async fn get_all(workspace_id: i64, pg: &PgPool) -> Result<Vec<Self>, AppError> {
        let chats = sqlx::query_as(
            r#"
            select id, name, type, workspace_id, user_ids, created_at
            from chat
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
            delete from chat
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppState;
    use chat_core::models::chat::Chat;

    #[tokio::test]
    async fn test_create_chat() -> Result<(), AppError> {
        let (tdb, _) = AppState::test_new().await?;
        let db = tdb.get_pool().await;

        let req = CreateChatReq {
            name: Some("test chat".to_string()),
            user_ids: vec![1, 2],
            is_public: false,
        };

        let chat = Chat::create(&req, 1, &db).await?;

        assert_eq!(chat.name, Some("test chat".to_string()));
        assert_eq!(chat.r#type, ChatType::Single);

        let req = CreateChatReq {
            name: Some("test chat1".to_string()),
            user_ids: vec![1, 2, 3],
            is_public: false,
        };
        let chat = Chat::create(&req, 1, &db).await?;

        assert_eq!(chat.name, Some("test chat1".to_string()));
        assert_eq!(chat.r#type, ChatType::Group);

        let req = CreateChatReq {
            name: Some("test chat2".to_string()),
            user_ids: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            is_public: false,
        };
        let chat = Chat::create(&req, 1, &db).await?;

        assert_eq!(chat.name, Some("test chat2".to_string()));
        assert_eq!(chat.r#type, ChatType::PrivateChannel);

        Ok(())
    }
}
