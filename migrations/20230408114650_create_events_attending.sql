-- Add migration script here
CREATE TABLE events_attending (
    user_id INT NOT NULL,
    event_id INT NOT NULL
);