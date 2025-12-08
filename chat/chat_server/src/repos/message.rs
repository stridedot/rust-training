use chat_core::{error::AppError, models::message::Message};
use sqlx::PgPool;

use crate::requests::message::{MessageListRequest, MessageSendRequest};

pub trait MessageRepo: Sized {
    fn create(
        chat_id: i64,
        sender_id: i64,
        req: MessageSendRequest,
        pg: &PgPool,
    ) -> impl std::future::Future<Output = Result<Self, AppError>> + Send;

    fn get_all(
        chat_id: i64,
        req: &MessageListRequest,
        pg: &PgPool,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, AppError>> + Send;
}

impl MessageRepo for Message {
    async fn create(
        chat_id: i64,
        sender_id: i64,
        req: MessageSendRequest,
        pg: &PgPool,
    ) -> Result<Message, AppError> {
        let msg = sqlx::query_as(
            r#"
            insert into message (chat_id, sender_id, content, files)
            values ($1, $2, $3, $4)
            returning id, chat_id, sender_id, content, files, created_at
            "#,
        )
        .bind(chat_id)
        .bind(sender_id)
        .bind(&req.content)
        .bind(&req.files)
        .fetch_one(pg)
        .await?;
        Ok(msg)
    }

    async fn get_all(
        chat_id: i64,
        req: &MessageListRequest,
        pg: &PgPool,
    ) -> Result<Vec<Self>, AppError> {
        let last_id = match req.last_id {
            Some(id) => id,
            None => i64::MAX as _,
        };
        let limit = req.limit.unwrap_or(10);

        let messages = sqlx::query_as(
            r#"
            select id, chat_id, sender_id, content, files, created_at
            from message
            where chat_id = $1
            and id < $2
            order by id desc
            limit $3
            "#,
        )
        .bind(chat_id)
        .bind(last_id)
        .bind(limit as i64)
        .fetch_all(pg)
        .await?;
        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use crate::AppState;

    use super::*;

    #[tokio::test]
    async fn test_message_create() -> Result<(), AppError> {
        let (tdb, _) = AppState::test_new().await?;
        let db = tdb.get_pool().await;

        let chat_id = 1;
        let sender_id = 1;
        let req = MessageSendRequest {
            content: "Hello, world!".to_string(),
            files: vec!["chat/files/1/7e1/139/697d00b564f9522765ed41f6b207b12de5.jpg".to_string()],
        };

        let msg = Message::create(chat_id, sender_id, req, &db).await?;
        assert_eq!(msg.chat_id, chat_id);
        assert_eq!(msg.sender_id, sender_id);
        assert_eq!(msg.content, "Hello, world!");
        assert_eq!(
            msg.files,
            vec!["chat/files/1/7e1/139/697d00b564f9522765ed41f6b207b12de5.jpg".to_string(),]
        );

        Ok(())
    }
}
