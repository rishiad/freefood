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
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES (?, ?, ?)
        "#,
            user.username,
            user.email,
            user.password
        )
        .execute(&mut tx)
        .await?;

        let row = sqlx::query!(
            r#"
            SELECT LAST_INSERT_ID() as id
        "#
        )
        .fetch_one(&mut tx)
        .await?;

        let id = match row.id {
            Some(value) => value as i32,
            None => return Err(sqlx::Error::RowNotFound),
        };
        tx.commit().await?;

        let user = self.get_user_by_id(id).await?;

        Ok(user)
    }

    async fn get_user_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT users.id, users.username, users.password_hash, users.email, users.created_at, users.updated_at, users.profile_picture, users.banner_picture, group_concat(distinct events_attending.event_id) as events_attending_ids, group_concat(distinct events_contributed.event_id) as events_contributed_ids, group_concat(distinct `following`.follower_id) as following_ids, group_concat(distinct followers.follower_id) as followers_ids
            FROM users
            LEFT JOIN events_attending ON events_attending.user_id = users.id
            LEFT JOIN events_contributed ON events_contributed.user_id = users.id
            LEFT JOIN `following` ON `following`.following_id = users.id
            LEFT JOIN followers ON followers.followed_id = users.id
            WHERE users.id = ?
            GROUP BY users.id;
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(user)
    }
    
    
    async fn get_user_by_username(&self, username: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT users.id, users.username, users.password_hash, users.email, users.created_at, users.updated_at, users.profile_picture, users.banner_picture, group_concat(distinct events_attending.event_id) as events_attending_ids, group_concat(distinct events_contributed.event_id) as events_contributed_ids, group_concat(distinct `following`.follower_id) as following_ids, group_concat(distinct followers.follower_id) as followers_ids
            FROM users
            LEFT JOIN events_attending ON events_attending.user_id = users.id
            LEFT JOIN events_contributed ON events_contributed.user_id = users.id
            LEFT JOIN `following` ON `following`.following_id = users.id
            LEFT JOIN followers ON followers.followed_id = users.id
            WHERE users.username = ?
            GROUP BY users.id;
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(user)
    }

    async fn get_user_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT users.id, users.username, users.password_hash, users.email, users.created_at, users.updated_at, users.profile_picture, users.banner_picture, group_concat(distinct events_attending.event_id) as events_attending_ids, group_concat(distinct events_contributed.event_id) as events_contributed_ids, group_concat(distinct `following`.follower_id) as following_ids, group_concat(distinct followers.follower_id) as followers_ids
            FROM users
            LEFT JOIN events_attending ON events_attending.user_id = users.id
            LEFT JOIN events_contributed ON events_contributed.user_id = users.id
            LEFT JOIN `following` ON `following`.following_id = users.id
            LEFT JOIN followers ON followers.followed_id = users.id
            WHERE users.email = ?
            GROUP BY users.id;
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(user)
    }

    async fn get_user_count(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM users
        "#
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(count)
    }

    async fn update_user(&self, user: User) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            "UPDATE users SET username = ?, email = ?, password_hash = ?, profile_picture = ?, banner_picture = ? WHERE id = ?",
            user.username, user.email, user.password_hash, user.profile_picture, user.banner_picture, user.id
        )
        .execute(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn delete_user(&self, id: i32) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            r#"
            DELETE FROM users WHERE id = ?
        "#,
            id
        )
        .execute(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }
}
