-- Migration to revert 'created_at' column back to TIMESTAMP
ALTER TABLE refresh_tokens
ALTER
COLUMN issued_at TYPE TIMESTAMP
USING issued_at AT TIME ZONE 'UTC';

ALTER TABLE refresh_tokens
ALTER
COLUMN expires TYPE TIMESTAMP
USING expires AT TIME ZONE 'UTC';

ALTER TABLE refresh_tokens
ALTER
COLUMN revocation_time TYPE TIMESTAMP
USING revocation_time AT TIME ZONE 'UTC';
