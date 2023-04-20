use crate::models::user::{NewUser, User};
use crate::repositories::user_repo::{UserRepository};
use async_trait::async_trait;
use sqlx::MySqlPool;
pub struct UserDAL {
    pool: MySqlPool,
}

impl UserDAL {
    pub fn new(pool: MySqlPool) -> Self {
        UserDAL { pool: pool }
    }
}

#[async_trait]

impl UserRepository for UserDAL {
    async fn create_user(&self, user: NewUser) -> Result<User, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        let result = sqlx::query!(
            "INSERT INTO users (first_name, last_name, email, password)
            VALUES (?, ?, ?, ?)",
            user.first_name,
            user.last_name,
            user.email,
            user.password
        )
        .execute(&mut conn)
        .await?;

        let user_id = result.last_insert_id() as i32;
        let created_user = self.get_user_by_id(user_id).await?;

        Ok(created_user)
    }

    async fn get_user_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        let user = sqlx::query!(
            "SELECT * FROM users WHERE id = ?",
            id
        )
        .fetch_one(&mut conn)
        .await?;

        Ok(User {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: user.password,
            created_at: user.created_at,
        })
    }

    async fn get_user_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        let user = sqlx::query!(
            "SELECT * FROM users WHERE email = ?",
            email
        )
        .fetch_one(&mut conn)
        .await?;

        Ok(User {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: user.password,
            created_at: user.created_at,
        })
    }
}
