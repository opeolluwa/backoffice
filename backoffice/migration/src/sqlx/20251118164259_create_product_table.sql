-- Add migration script here
CREATE TABLE IF NOT EXISTS products (
    identifier CHAR(26) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    picture VARCHAR,
    price NUMERIC(12, 2) NOT NULL,
    description TEXT NOT NULL,
    created_by_identifier CHAR(26) REFERENCES users(identifier),
    marketplace_identifier CHAR(26) REFERENCES marketplaces(identifier),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NULL
);

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;

$$ LANGUAGE plpgsql;
CREATE TRIGGER update_products_updated_at BEFORE
UPDATE ON products FOR EACH ROW EXECUTE FUNCTION set_updated_at();