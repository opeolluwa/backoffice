-- Add migration script here
ALTER TABLE users
    ADD COLUMN role_identifier CHAR(26) REFERENCES user_roles (identifier);