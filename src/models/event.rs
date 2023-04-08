use serde::{Serialize};
use crate::models::user::User;


#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
    pub contributor: User,
    pub location: String, 
    pub free_food: bool
}