use crate::models::event::Event;

pub trait EventRepository {
    fn create_event(&self, event: Event) -> Result<Event, sqlx::Error>;
    fn get_event_by_id(&self, id: i32) -> Result<Event, sqlx::Error>;
    fn get_event_by_name(&self, name: String) -> Result<Event, sqlx::Error>;
    fn get_event_count(&self) -> Result<i64, sqlx::Error>;
    fn update_event(&self, event: Event) -> Result<Event, sqlx::Error>;
    fn delete_event(&self, id: i32) -> Result<(), sqlx::Error>;
}