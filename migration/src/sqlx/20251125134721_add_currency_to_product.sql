-- Add migration script here
ALTER TABLE products
ADD COLUMN currency_identifier CHAR(26) REFERENCES countries(identifier)