-- Migration to change 'created_at' column to TIMESTAMPTZ
ALTER TABLE accounts
ALTER COLUMN created_at TYPE TIMESTAMPTZ
USING created_at AT TIME ZONE 'UTC';