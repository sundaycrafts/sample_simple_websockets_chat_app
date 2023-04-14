use crate::models::user::User;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Messenger {
    async fn send(&self, msg: &str, users: Vec<User>) -> Result<(), Box<dyn Error>>;
}
