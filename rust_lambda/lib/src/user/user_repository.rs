use async_trait::async_trait;

use crate::errors::errors::Error;

use super::user_type::User;
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> Result<(), Error>;
    async fn find_by_id(&self, id: &str) -> Result<User, Error>;
}
