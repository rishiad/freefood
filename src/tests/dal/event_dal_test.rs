use chrono::Utc;
use crate::{dal::event_dal::EventDAL, models::Event, tests::dal::{setup, teardown}, repositories::event_repo::EventRepository};


#[tokio::test]
async fn test_create_event() {
    let tx = setup().await.unwrap();
    let event_dal = EventDAL::new(tx.1);

    let event = Event {
        id: 0,
        name: String::from("Test Event"),
        description: String::from("This is a test event."),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        contributor_id: 1,
        location: String::from("Test Location"),
        free_food: true,
        start_time: Utc::now(),
        end_time: Utc::now(),
        users_attending: vec![],
    };

    let created_event = event_dal.create_event(event.clone()).await.unwrap();

    assert_eq!(&event.name, &created_event.name);
    assert_eq!(&event.description, &created_event.description);
    assert_eq!(&event.contributor_id, &created_event.contributor_id);
    assert_eq!(&event.location, &created_event.location);
    assert_eq!(&event.free_food, &created_event.free_food);
    assert_eq!(&event.start_time, &created_event.start_time);
    assert_eq!(&event.end_time, &created_event.end_time);
    assert_eq!(&event.users_attending, &created_event.users_attending);

    teardown(tx.0).await.unwrap();
}

#[tokio::test]
async fn test_get_event_by_id() {
    let tx = setup().await.unwrap();
    let event_dal = EventDAL::new(tx.1);

    let event = Event {
        id: 0,
        name: String::from("Test Event"),
        description: String::from("This is a test event."),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        contributor_id: 1,
        location: String::from("Test Location"),
        free_food: true,
        start_time: Utc::now(),
        end_time: Utc::now(),
        users_attending: vec![],
    };

    let created_event = event_dal.create_event(event.clone()).await.unwrap();

    let retrieved_event = event_dal.get_event_by_id(created_event.id).await.unwrap();

    assert_eq!(&created_event.name, &retrieved_event.name);
    assert_eq!(&created_event.description, &retrieved_event.description);
    assert_eq!(&created_event.contributor_id, &retrieved_event.contributor_id);
    assert_eq!(&created_event.location, &retrieved_event.location);
    assert_eq!(&created_event.free_food, &retrieved_event.free_food);
    assert_eq!(&created_event.start_time, &retrieved_event.start_time);
    assert_eq!(&created_event.end_time, &retrieved_event.end_time);
    assert_eq!(&created_event.users_attending, &retrieved_event.users_attending);

    teardown(tx.0).await.unwrap();
}

#[tokio::test]
async fn test_get_event_by_name() {
    let tx = setup().await.unwrap();
    let event_dal = EventDAL::new(tx.1);

    let event = Event {
        id: 0,
        name: String::from("Test Event"),
        description: String::from("This is a test event."),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        contributor_id: 1,
        location: String::from("Test Location"),
        free_food: true,
        start_time: Utc::now(),
        end_time: Utc::now(),
        users_attending: vec![],
    };

    let created_event = event_dal.create_event(event.clone()).await.unwrap();

    let retrieved_event = event_dal.get_event_by_name(created_event.clone().name).await.unwrap();

    assert_eq!(&created_event.name, &retrieved_event.name);
    assert_eq!(&created_event.description, &retrieved_event.description);
    assert_eq!(&created_event.contributor_id, &retrieved_event.contributor_id);
    assert_eq!(&created_event.location, &retrieved_event.location);
    assert_eq!(&created_event.free_food, &retrieved_event.free_food);
    assert_eq!(&created_event.start_time, &retrieved_event.start_time);
    assert_eq!(&created_event.end_time, &retrieved_event.end_time);
    assert_eq!(&created_event.users_attending, &retrieved_event.users_attending);

    teardown(tx.0).await.unwrap();
}

#[tokio::test]
async fn get_event_count() {
    let tx = setup().await.unwrap();
    let event_dal = EventDAL::new(tx.1);

    let event = Event {
        id: 0,
        name: String::from("Test Event"),
        description: String::from("This is a test event."),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        contributor_id: 1,
        location: String::from("Test Location"),
        free_food: true,
        start_time: Utc::now(),
        end_time: Utc::now(),
        users_attending: vec![],
    };

    let created_event = event_dal.create_event(event.clone()).await.unwrap();

    let count = event_dal.get_event_count().await.unwrap();

    assert_eq!(count, 1);

    teardown(tx.0).await.unwrap();
}

#[tokio::test]
async fn update_event() {
    let tx = setup().await.unwrap();
    let event_dal = EventDAL::new(tx.1);

    let event = Event {
        id: 0,
        name: String::from("Test Event"),
        description: String::from("This is a test event."),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        contributor_id: 1,
        location: String::from("Test Location"),
        free_food: true,
        start_time: Utc::now(),
        end_time: Utc::now(),
        users_attending: vec![],
    };

    let created_event = event_dal.create_event(event.clone()).await.unwrap();

    let updated_event = Event {
        id: created_event.id,
        name: String::from("Updated Test Event"),
        description: String::from("This is an updated test event."),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        contributor_id: 1,
        location: String::from("Updated Test Location"),
        free_food: false,
        start_time: Utc::now(),
        end_time: Utc::now(),
        users_attending: vec![],
    };

    let updated_event = event_dal.update_event(updated_event.clone()).await.unwrap();

    assert_eq!(&updated_event.name, &created_event.name);
    assert_eq!(&updated_event.description, &created_event.description);
    assert_eq!(&updated_event.contributor_id, &created_event.contributor_id);
    assert_eq!(&updated_event.location, &created_event.location);
    assert_eq!(&updated_event.free_food, &created_event.free_food);
    assert_eq!(&updated_event.start_time, &created_event.start_time);
    assert_eq!(&updated_event.end_time, &created_event.end_time);
    assert_eq!(&updated_event.users_attending, &created_event.users_attending);

    teardown(tx.0).await.unwrap();
}

#[tokio::test]
async fn delete_event() {
    let tx = setup().await.unwrap();
    let event_dal = EventDAL::new(tx.1);

    let event = Event {
        id: 0,
        name: String::from("Test Event"),
        description: String::from("This is a test event."),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        contributor_id: 1,
        location: String::from("Test Location"),
        free_food: true,
        start_time: Utc::now(),
        end_time: Utc::now(),
        users_attending: vec![],
    };

    let created_event = event_dal.create_event(event.clone()).await.unwrap();

    let deleted_event = event_dal.delete_event(created_event.id).await.unwrap();
    
    let check_event = event_dal.get_event_by_id(created_event.id).await;

    if check_event.is_ok() {
        panic!("Event was not deleted");
    } 
    teardown(tx.0).await.unwrap();
}