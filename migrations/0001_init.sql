CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    note TEXT NOT NULL
)
-- migrations/20210907123456_create_users_table.sql

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);