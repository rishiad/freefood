-- Add migration script here
CREATE TABLE events_contributed (
    user_id INT NOT NULL,
    event_id INT NOT NULL
);