-- Add migration script here
CREATE TABLE IF NOT EXISTS activities (
    identifier CHAR(26) NOT NULL,
    created_by_identifier CHAR(26) REFERENCES users(identifier),
    resource VARCHAR NOT NULL,
    action VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NULL
)