use serde::{Serialize};
use crate::models::user::User;


#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub contributor_id: i32,
    pub location: String,
    pub free_food: bool,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub users_attending: Vec<i32>,
}

