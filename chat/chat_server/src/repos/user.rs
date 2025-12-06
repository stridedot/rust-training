use std::mem;

use chat_core::{
    error::AppError,
    models::{
        user::{ChatUser, User},
        workspace::Workspace,
    },
};
use sqlx::PgPool;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use crate::{
    repos::workspace::WorkspaceRepo as _,
    requests::{
        user::{SignInReq, SignUpReq},
        workspace::CreateWorkspaceReq,
    },
};

pub trait UserRepo: Sized {
    fn create(
        input: &SignUpReq,
        db: &PgPool,
    ) -> impl std::future::Future<Output = Result<Self, AppError>> + Send;

    fn find_by_email(
        email: &str,
        db: &PgPool,
    ) -> impl std::future::Future<Output = Result<Option<Self>, AppError>> + Send;

    fn verify(
        input: &SignInReq,
        db: &PgPool,
    ) -> impl std::future::Future<Output = Result<Option<Self>, AppError>> + Send;

    fn get_all(
        db: &PgPool,
    ) -> impl std::future::Future<Output = Result<Vec<ChatUser>, AppError>> + Send;

    fn find_by_ids(
        ids: &[i64],
        db: &PgPool,
    ) -> impl std::future::Future<Output = Result<Vec<ChatUser>, AppError>> + Send;
}

impl UserRepo for User {
    async fn create(input: &SignUpReq, db: &PgPool) -> Result<Self, AppError> {
        let workspace = match Workspace::find_by_name(&input.workspace, db).await? {
            Some(workspace) => workspace,
            None => {
                Workspace::create(
                    &CreateWorkspaceReq {
                        name: input.workspace.clone(),
                        owner_id: 0,
                    },
                    db,
                )
                .await?
            }
        };

        let user = sqlx::query_as(
            r#"
            insert into users
            (username, email, password, workspace_id)
            values ($1, $2, $3, $4)
            returning id, username, email, created_at
            "#,
        )
        .bind(&input.username)
        .bind(&input.email)
        .bind(hash_password(&input.password)?)
        .bind(workspace.id)
        .fetch_one(db)
        .await?;

        Ok(user)
    }

    async fn find_by_email(email: &str, db: &PgPool) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as(
            r#"
            select id, username, email, workspace_id, created_at
            from users
            where email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(db)
        .await?;

        Ok(user)
    }

    async fn verify(input: &SignInReq, db: &PgPool) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as(
            r#"
            select id, username, email, password, workspace_id, created_at
            from users
            where email = $1
            "#,
        )
        .bind(&input.email)
        .fetch_optional(db)
        .await?;

        if let Some(mut user) = user {
            let password_hash = mem::take(&mut user.password);
            if verify_password(&input.password, &password_hash)? {
                return Ok(Some(user));
            }
        }

        Ok(None)
    }

    async fn get_all(db: &PgPool) -> Result<Vec<ChatUser>, AppError> {
        let users = sqlx::query_as(
            r#"
            select id, username, email, workspace_id, created_at
            from users
            "#,
        )
        .fetch_all(db)
        .await?;

        Ok(users)
    }

    async fn find_by_ids(ids: &[i64], db: &PgPool) -> Result<Vec<ChatUser>, AppError> {
        let users = sqlx::query_as(
            r#"
            select id, username, email, workspace_id, created_at
            from users
            where id = any($1)
            "#,
        )
        .bind(ids)
        .fetch_all(db)
        .await?;

        Ok(users)
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(hashed_password.to_string())
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(password_hash)?;
    let is_valid = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use crate::AppState;

    use super::*;

    use crate::requests::user::{SignInReq, SignUpReq};
    use chat_core::models::user::User;

    #[tokio::test]
    async fn test_create_user() -> Result<(), AppError> {
        let (tdb, _) = AppState::test_new().await?;
        let db = tdb.get_pool().await;

        let input = SignUpReq {
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "test".to_string(),
            workspace: "test".to_string(),
        };
        let user = User::create(&input, &db).await?;
        assert_eq!(user.username, input.username);
        assert_eq!(user.email, input.email);

        let user = User::find_by_email(&input.email, &db)
            .await?
            .expect("user not found");
        assert_eq!(user.username, input.username);

        let input = SignInReq {
            email: input.email.clone(),
            password: input.password.clone(),
        };

        let user = User::verify(&input, &db).await?.expect("user not found");
        assert_eq!(user.email, input.email);

        Ok(())
    }
}
