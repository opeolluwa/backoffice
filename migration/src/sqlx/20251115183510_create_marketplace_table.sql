-- Add migration script here
CREATE TABLE IF NOT EXISTS marketplace (
    identifier CHAR(26) PRIMARY KEY,
    name VARCHAR NOT NULL,
    slug VARCHAR UNIQUE NOT NULL,
    description VARCHAR NOT NULL,
    user_identifier CHAR(26) REFERENCES users(identifier),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NULL
)