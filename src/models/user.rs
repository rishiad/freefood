use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use validator::{Validate};


#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub events_attending_ids: Option<String>,
    pub events_contributed_ids: Option<String>,
    pub following_ids: Option<String>,
    pub followers_ids: Option<String>,
    pub profile_picture: Option<String>,
    pub banner_picture: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(length(min = 8, max = 100))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    pub profile_picture: Option<String>,
    pub banner_picture: Option<String>,
}

