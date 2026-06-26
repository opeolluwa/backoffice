CREATE TABLE uploads (
    identifier  CHAR(26)     NOT NULL PRIMARY KEY,
    name        TEXT         NOT NULL,
    src         TEXT         NOT NULL,
    file_type   TEXT,
    size        BIGINT,
    starred     BOOLEAN      NOT NULL DEFAULT false,
    user_identifier CHAR(26) REFERENCES users(identifier) ON DELETE SET NULL,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ
);

CREATE OR REPLACE FUNCTION update_uploads_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER uploads_updated_at_trigger
BEFORE UPDATE ON uploads
FOR EACH ROW
EXECUTE FUNCTION update_uploads_updated_at();
