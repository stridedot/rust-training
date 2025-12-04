use chat_core::{error::AppError, models::workspace::Workspace};
use sqlx::PgPool;

use crate::requests::workspace::CreateWorkspaceReq;

pub trait WorkspaceRepo: Sized {
    fn create(
        input: &CreateWorkspaceReq,
        db: &PgPool,
    ) -> impl std::future::Future<Output = Result<Self, AppError>> + Send;

    fn find_by_name(
        name: &str,
        db: &PgPool,
    ) -> impl std::future::Future<Output = Result<Option<Self>, AppError>> + Send;

    fn update_owner(
        &self,
        owner_id: i64,
        db: &PgPool,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
}

impl WorkspaceRepo for Workspace {
    async fn create(input: &CreateWorkspaceReq, db: &PgPool) -> Result<Self, AppError> {
        let workspace = sqlx::query_as(
            r#"
            insert into workspace (name)
             values ($1) returning id, name, owner_id, created_at
            "#,
        )
        .bind(&input.name)
        .fetch_one(db)
        .await?;

        Ok(workspace)
    }

    async fn find_by_name(name: &str, db: &PgPool) -> Result<Option<Self>, AppError> {
        let workspace = sqlx::query_as(
            r#"
            select *
            from workspace
            where name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(db)
        .await?;

        Ok(workspace)
    }

    async fn update_owner(&self, owner_id: i64, db: &PgPool) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            update workspace
            set owner_id = $1
            where id = $2
            "#,
        )
        .bind(owner_id)
        .bind(self.id)
        .execute(db)
        .await?;

        match result.rows_affected() {
            0 => Err(AppError::NotFound(format!(
                "workspace with name {} not found",
                self.name
            ))),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::AppState;

    use super::*;

    #[tokio::test]
    async fn test_create_workspace() -> Result<(), AppError> {
        let (_, state) = AppState::test_new().await?;
        let _ = &state.inner.pg_pool;

        Ok(())
    }
}
