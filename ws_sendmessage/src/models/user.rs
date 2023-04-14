use async_trait::async_trait;
use std::error::Error;

pub struct User {
    pub connection_id: String,
}

impl User {
    pub fn new(connection_id: &str) -> Self {
        Self {
            connection_id: connection_id.into(),
        }
    }
}

#[async_trait]
pub trait UserRepository {
    async fn users_all(&self) -> Result<Vec<User>, Box<dyn Error>>;
}
