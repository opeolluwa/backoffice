-- Add migration script here
CREATE TABLE IF NOT EXISTS products (
    identifier CHAR(26) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    picture VARCHAR,
    price NUMERIC(12, 2) NOT NULL,
    description TEXT NOT NULL,
    created_by_identifier CHAR(26) REFERENCES users(identifier),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NULL
);