-- Add migration script here
CREATE TABLE followers (
    follower_id INT NOT NULL,
    followed_id INT NOT NULL
);