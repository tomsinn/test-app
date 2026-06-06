-- Create User Table
CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

-- Populate Users Table
INSERT INTO users (name)
VALUES ('Alice'), ('Bob'), ('Charlie');
