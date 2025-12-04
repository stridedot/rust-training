use crate::models::user::User;

pub mod auth;

pub trait TokenVerify {
    type Error: std::fmt::Debug;

    fn verify(&self, token: &str) -> Result<User, Self::Error>;
}
