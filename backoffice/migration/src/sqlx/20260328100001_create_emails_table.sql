CREATE TABLE emails (
    identifier      CHAR(26)    NOT NULL PRIMARY KEY,
    subject         TEXT        NOT NULL,
    body            TEXT        NOT NULL,
    sender_email    TEXT        NOT NULL,
    recipient_email TEXT        NOT NULL,
    date_sent       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tag             TEXT,
    is_read         BOOLEAN     NOT NULL DEFAULT false,
    is_starred      BOOLEAN     NOT NULL DEFAULT false,
    has_attachments BOOLEAN     NOT NULL DEFAULT false,
    data            JSONB,
    user_identifier CHAR(26)    REFERENCES users(identifier) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ
);

CREATE OR REPLACE FUNCTION update_emails_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER emails_updated_at_trigger
BEFORE UPDATE ON emails
FOR EACH ROW
EXECUTE FUNCTION update_emails_updated_at();
