-- Add migration script here
-- Add migration script here
CREATE TABLE events (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    contributor_id INT NOT NULL,
    location VARCHAR(255) NOT NULL,
    free_food BOOLEAN NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    users_attending VARCHAR(255) NOT NULL
    );