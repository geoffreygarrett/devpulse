-- Migration to change 'created_at' column to TIMESTAMPTZ
ALTER TABLE refresh_tokens
ALTER
COLUMN issued_at TYPE TIMESTAMPTZ
USING issued_at AT TIME ZONE 'UTC';

ALTER TABLE refresh_tokens
ALTER
COLUMN expires TYPE TIMESTAMPTZ
USING expires AT TIME ZONE 'UTC';

ALTER TABLE refresh_tokens
ALTER
COLUMN revocation_time TYPE TIMESTAMPTZ
USING revocation_time AT TIME ZONE 'UTC';