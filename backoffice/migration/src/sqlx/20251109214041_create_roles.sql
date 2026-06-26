-- Add migration script here

CREATE TABLE user_roles
(
    identifier  CHAR(26) PRIMARY KEY,
    name        VARCHAR     NOT NULL,
    description VARCHAR              DEFAULT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    update_at   TIMESTAMPTZ
)