-- Add migration script here
CREATE TABLE following (
    follower_id INT NOT NULL,
    following_id INT NOT NULL
);