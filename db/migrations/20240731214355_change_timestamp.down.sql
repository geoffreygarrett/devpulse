-- Down migration to revert 'created_at' column from TIMESTAMPTZ to TIMESTAMP
ALTER TABLE accounts
ALTER COLUMN created_at TYPE TIMESTAMP
USING created_at AT TIME ZONE 'UTC';
