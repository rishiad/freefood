use crate::models::Event;
use crate::repositories::event_repo::EventRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc, NaiveDateTime};
use sqlx::MySqlPool;

pub struct EventDAL {
    pool: MySqlPool,
}

impl EventDAL {
    pub fn new(pool: MySqlPool) -> Self {
        EventDAL { pool: pool }
    }
}

#[async_trait]
impl EventRepository for EventDAL {

    async fn create_event(&self, event: Event) -> Result<Event, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        let users_attending_json = serde_json::to_string(&event.users_attending).unwrap();

        let result = sqlx::query!(
            "INSERT INTO events (name, description, contributor_id, location, free_food, start_time, end_time, users_attending)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            event.name,
            event.description,
            event.contributor_id,
            event.location,
            event.free_food,
            event.start_time,
            event.end_time,
            users_attending_json.as_bytes()
        )
        .execute(&mut conn)
        .await?;

        let event_id = result.last_insert_id() as i32;
        let created_event = self.get_event_by_id(event_id).await?;

        Ok(created_event)
    }

    async fn get_event_by_id(&self, id: i32) -> Result<Event, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        let event = sqlx::query!(
            "SELECT * FROM events WHERE id = ?",
            id
        )
        .fetch_one(&mut conn)
        .await?;

        let users_attending: Vec<i32> = serde_json::from_str(&event.users_attending).unwrap();

        Ok(Event {
            id: event.id,
            name: event.name,
            description: event.description,
            created_at: event.created_at,
            updated_at: event.updated_at,
            contributor_id: event.contributor_id,
            location: event.location,
            free_food: event.free_food != 0,
            start_time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(event.start_time.timestamp(), 0), Utc),
            end_time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(event.end_time.timestamp(), 0), Utc),
            users_attending: users_attending,
        })
    }

    async fn get_event_by_name(&self, name: String) -> Result<Event, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        let event = sqlx::query!(
            "SELECT * FROM events WHERE name = ?",
            name
        )
        .fetch_one(&mut conn)
        .await?;

        let users_attending: Vec<i32> = serde_json::from_str(&event.users_attending).unwrap();

        Ok(Event {
            id: event.id,
            name: event.name,
            description: event.description,
            created_at: event.created_at,
            updated_at: event.updated_at,
            contributor_id: event.contributor_id,
            location: event.location,
            free_food: event.free_food != 0,
            start_time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(event.start_time.timestamp(), 0), Utc),
            end_time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(event.end_time.timestamp(), 0), Utc),
            users_attending: users_attending,
        })
    }

    async fn get_event_count(&self) -> Result<i64, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM events"
        )
        .fetch_one(&mut conn)
        .await?;

        Ok(count.count)
    }

    async fn update_event(&self, event: Event) -> Result<Event, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        let users_attending_json = serde_json::to_string(&event.users_attending).unwrap();

        sqlx::query!(
            "UPDATE events SET name = ?, description = ?, contributor_id = ?, location = ?, free_food = ?, start_time = ?, end_time = ?, users_attending = ? WHERE id = ?",
            event.name,
            event.description,
            event.contributor_id,
            event.location,
            event.free_food,
            event.start_time,
            event.end_time,
            users_attending_json.as_bytes(),
            event.id
        )
        .execute(&mut conn)
        .await?;

        let updated_event = self.get_event_by_id(event.id).await?;

        Ok(updated_event)
    }

    async fn delete_event(&self, id: i32) -> Result<(), sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        sqlx::query!(
            "DELETE FROM events WHERE id = ?",
            id
        )
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    
}
