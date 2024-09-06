CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    note TEXT NOT NULL
);
-- migrations/20210907123456_create_users_table.sql

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);