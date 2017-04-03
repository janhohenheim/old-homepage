CREATE TABLE user_account (
    id SERIAL PRIMARY KEY,
    email VARCHAR(254) NOT NULL UNIQUE,
    name VARCHAR(254) NOT NULL UNIQUE,
    password TEXT NOT NULL
);