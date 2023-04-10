use crate::models::event::Event;
use async_trait::async_trait;

#[async_trait]
pub trait EventRepository {
    async fn create_event(&self, event: Event) -> Result<Event, sqlx::Error>;
    async fn get_event_by_id(&self, id: i32) -> Result<Event, sqlx::Error>;
    async fn get_event_by_name(&self, name: String) -> Result<Event, sqlx::Error>;
    async fn get_event_count(&self) -> Result<i64, sqlx::Error>;
    async fn update_event(&self, event: Event) -> Result<Event, sqlx::Error>;
    async fn delete_event(&self, id: i32) -> Result<(), sqlx::Error>;
}