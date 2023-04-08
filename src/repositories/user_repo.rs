
use crate::models::user::{User, NewUser};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: NewUser) -> Result<User, sqlx::Error>;
    async fn get_user_by_id(&self, id: i32) -> Result<User, sqlx::Error>;
    async fn get_user_by_username(&self, username: String) -> Result<User, sqlx::Error>;
    async fn get_user_by_email(&self, email: String) -> Result<User, sqlx::Error>;
    async fn get_user_count(&self) -> Result<i64, sqlx::Error>;
    async fn update_user(&self, user: User) -> Result<(), sqlx::Error>;
    async fn delete_user(&self, id: i32) -> Result<(), sqlx::Error>;
}
