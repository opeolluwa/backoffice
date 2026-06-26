-- Add migration script here
CREATE TABLE IF NOT EXISTS app_config (
    identifier SMALLINT PRIMARY KEY CHECK (identifier = 1),
    app_name TEXT,
    maintenance_mode BOOLEAN NOT NULL DEFAULT FALSE,
    support_email TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);